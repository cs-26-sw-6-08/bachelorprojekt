use std::error::Error;

use crate::{monitor::{streams::{IoTDevice, IoTStream, OutputStream}, types::{StackContent, Verdict}}, monitor_setup::operation_types::DerivedStream};

impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) -> Result<(), Box<dyn Error>> {
        for (t_spawn, ver) in self.unresolved_timepoints.iter_mut() {
            let res = eval_operations(&mut self.derived_streams, devices, &*t_spawn, &t_current);
            let res = res?;
            let res_val = res.get_value().get_verdict()?;
            //Set verdict
            if !res_val {
                // if !res_val && res.is_decided() {
                *ver = Verdict::False;
            } else if res.is_decided() {
                *ver = Verdict::True;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
enum StepType {
    Deepen,
    Reduce,
    ReducePartial,
}
#[derive(Debug)]
enum DeviceStack<'a> {
    Element(&'a IoTDevice),
    LayerShift,
}

impl<'a> From<&'a IoTDevice> for DeviceStack<'a> {
    fn from(value: &'a IoTDevice) -> Self {
        DeviceStack::Element(value)
    }
}

pub(crate) fn eval_operations<'a>(
    operations: &mut [DerivedStream],
    devices: &'a IoTStream,
    t_spawn: &i128,
    t_current: &i128,
    //todo: Should this return verdict ???
) -> Result<Verdict, Box<dyn Error>> {
    todo!();
    use StepType::*;

    let mut worklist_stack: Vec<(usize, StepType)> = Vec::with_capacity(50);
    let mut value_stack: Vec<StackContent> = Vec::with_capacity(50);
    let mut device_stack: Vec<DeviceStack> = Vec::with_capacity(50);
    let mut device_pointer: Option<&IoTDevice> = None;
    let mut time_offset_stack: Vec<(i128, i128)> = Vec::with_capacity(50);

    worklist_stack.push((0usize, StepType::Deepen));

    while let Some((cur_idx, step_type)) = worklist_stack.pop() {
        let cur_op = &mut operations[cur_idx] as *mut DerivedStream;

        match (unsafe { &mut *cur_op }, step_type) {
            // Base cases
            (DerivedStream::Number(val), _) => value_stack.push((*val).into()),
            (DerivedStream::String(str), _) => value_stack.push((&*str).into()),
            (DerivedStream::SpawnTime, _) => value_stack.push(
                (time_offset_stack
                    .last()
                    .map(|(ts, _)| ts)
                    .unwrap_or(t_spawn)
                    * 1_000)
                    .into(),
            ),
            (DerivedStream::Member(mem_type), _) => {
                value_stack.push(match mem_type {
                    MemberType::Power => device_pointer
                        .ok_or(errors::Error::DevicePointer)?
                        .power
                        .into(),
                    MemberType::Name => StreamOutput::from(
                        device_pointer
                            .map(|d| &d.name)
                            .ok_or(errors::Error::DevicePointer)?,
                    ),
                });
            }
            // BinOp / UnOp
            (DerivedStream::Binary { idx_lhs, .. }, Deepen) => {
                worklist_stack.extend([(cur_idx, ReducePartial), (*idx_lhs, Deepen)]);
            }
            (
                DerivedStream::Binary {
                    bin_op, idx_rhs, ..
                },
                ReducePartial,
            ) => {
                //If the binary operation is an 'or' and returned true, then the rest shouldn't be evaluated
                // Read as: 'or' -> last_val.is_false && last_val.is_decided
                if !matches!(bin_op, BinaryOperators::Or)
                    || !value_stack.last().is_some_and(|val| {
                        matches!(*val.get_value(), StackContent::Verdict(true)) && val.is_decided()
                    })
                {
                    worklist_stack.extend([(cur_idx, Reduce), (*idx_rhs, Deepen)]);
                }
            }
            (DerivedStream::Binary { bin_op, .. }, Reduce) => {
                let v_rhs = value_stack.pop_or_err()?;
                let v_lhs = value_stack.pop_or_err()?;
                value_stack.push(v_lhs.bin_op(v_rhs, bin_op));
            }
            (DerivedStream::Unary { idx, .. }, Deepen) => {
                worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
            }
            (DerivedStream::Unary { un_op, .. }, Reduce) => {
                let res = value_stack.pop_or_err()?.un_op(un_op);
                value_stack.push(res);
            }

            // Aggregate Functions
            (DerivedStream::AggregateFunction { .. }, Deepen) => {
                worklist_stack.extend([(cur_idx, ReducePartial)]);

                //Put devices on device stack and pop the first
                device_stack.push(DeviceStack::LayerShift);
                for d in devices.get_devices(cur_idx) {
                    device_stack.push(d.into());
                }
                //Accumulation starts at zero
                value_stack.push(0.into());
                value_stack.push(0.into());
            }
            (DerivedStream::AggregateFunction { idx, .. }, ReducePartial) => {
                //Pop the accumulated value and newest value on the stack and add them
                let res = value_stack.pop_or_err()? + value_stack.pop_or_err()?;
                value_stack.push(res);

                match device_stack.pop() {
                    Some(DeviceStack::Element(device)) => {
                        device_pointer = Some(device);
                        worklist_stack.extend([(cur_idx, ReducePartial), (*idx, Deepen)]);
                    }
                    Some(DeviceStack::LayerShift) | None => worklist_stack.push((cur_idx, Reduce)),
                }
            }
            (DerivedStream::AggregateFunction { function_type, .. }, Reduce) => {
                let res = value_stack.pop_or_err()?;
                value_stack.push(match function_type {
                    AggregateType::Sum => res,
                    AggregateType::Avg => res / (devices.get_devices(cur_idx).len() as i128).into(),
                });
            }
            (DerivedStream::Foreach { .. }, Deepen) => {
                worklist_stack.push((cur_idx, Reduce));
                device_stack.push(DeviceStack::LayerShift);
                for d in devices.get_devices(cur_idx) {
                    device_stack.push(d.into());
                }
                value_stack.push(true.into())
            }
            (DerivedStream::Foreach { idx }, Reduce) => {
                //Violation didn't occur and not all devices have been looked at
                if value_stack
                    .last()
                    .is_some_and(|v| matches!(*v.get_value(), StackContent::Verdict(true)))
                    && !device_stack
                        .last()
                        .is_some_and(|v| matches!(v, DeviceStack::LayerShift))
                {
                    let _ = value_stack.pop();
                    device_pointer = match device_stack.pop() {
                        Some(DeviceStack::Element(v)) => Some(v),
                        Some(DeviceStack::LayerShift) | None => unreachable!(),
                    };
                    worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);

                //If here, then a violation occured or not depending on the last value in value_stack
                } else {
                    while let Some(DeviceStack::Element(_)) = device_stack.pop() {}
                }
            }
            // Time functions
            (
                DerivedStream::TimeFunction {
                    idx,
                    bound,
                    history,
                    function_type,
                },
                Deepen,
            ) => {
                let (t_lower, _) = time_offset_stack
                    .last()
                    .map(|(v1, v2)| (*v1, *v2))
                    .unwrap_or((*t_spawn, *t_spawn));
                //If bound has already been exceeded we aren't interested in calculating further
                match bound {
                    //The difference between t_c and t_s is the time the bound has been active.
                    //If it exceeds the end (b) (added 1 because of it the num being inclusive), then it shouldn't evaluate the expression and it is decided (or untainted)
                    b if (*t_current - t_lower) == *b + 1 => {
                        let prev_val = history[(t_spawn % (*b + 1)) as usize].value;
                        value_stack
                            .push(function_type_computation(function_type, prev_val, *b).into());
                    }
                    _ => worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]),
                }
            }
            (
                DerivedStream::TimeFunction {
                    function_type,
                    history,
                    bound,
                    ..
                },
                Reduce,
            ) => {
                let val = value_stack.pop_or_err()?.get_value().get_num()?;
                let val = time_function_reduce_step(val, *t_spawn, *bound, history);
                let val: StreamOutput =
                    function_type_computation(function_type, val, *bound).into();
                value_stack.push(val.to_undecided());
            }

            // LTL
            (DerivedStream::LTLAlwaysUnbounded { idx }, Deepen) => {
                worklist_stack.push((*idx, Deepen));
            }
            (
                DerivedStream::LTLBounded {
                    idx,
                    bound,
                    ltl_type,
                    ..
                },
                Deepen,
            ) => {
                let (a, b) = bound;
                let (t_lower, t_upper) = time_offset_stack
                    .last()
                    .map(|(v1, v2)| (*v1, *v2))
                    .unwrap_or((*t_spawn, *t_spawn));
                //If over bound, should add verdict to stack and move back up
                //fst is lowerbound, snd is upperbound
                match (
                    *a + t_lower <= *t_current,
                    *t_current <= t_upper + *b,
                    ltl_type,
                ) {
                    //Bound has not been entered yet
                    (false, true, _) => value_stack.push(StreamOutput::from(true).to_undecided()),
                    //Within Bound For Always
                    (true, true, ExprLTL::Always) => {
                        time_offset_stack.push((*a + t_lower, *b + t_upper));
                        worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)])
                    }
                    //Bound has been passed For always
                    (true, false, ExprLTL::Always) => {
                        time_offset_stack.push((*a + t_lower, *b + t_upper));
                        worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)])
                    }

                    //Within Bound For Eventually
                    (true, true, ExprLTL::Eventually(his)) => {
                        let his_idx = (*t_spawn % (*b - *a + 1)) as usize;
                        match his.get(his_idx) {
                            //If previous value that corresponds to the spawn point gave true previously,
                            //then we shouldn't look further down in the tree
                            Some(val) if val.value && val.spawn_point == *t_spawn => {
                                value_stack.push(true.into())
                            }
                            None | Some(_) => {
                                time_offset_stack.push((*a + t_lower, *b + t_upper));
                                worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)])
                            }
                        }
                    }
                    //Bound has been passed For Eventually
                    // TODO: Den skal ændres, så den ikke bare blindt siger push true
                    (true, false, ExprLTL::Eventually(_)) => {
                        time_offset_stack.push((*a + t_lower, *b + t_upper));
                        worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                    }
                    //Unreachable case -> Can't be below bound and above bound at same time
                    (false, false, _) => unreachable!(),
                }
            }
            (
                DerivedStream::LTLBounded {
                    not,
                    bound,
                    ltl_type,
                    ..
                },
                Reduce,
            ) => {
                let (_, t_upper) = time_offset_stack.pop_or_err()?;

                match ltl_type {
                    ExprLTL::Always => {
                        let val = value_stack.pop_or_err()?;

                        let val = if *t_current < t_upper {
                            //Undecideable when here -> As the bound haven't been reached yet
                            val.to_undecided()
                        } else {
                            val
                        };

                        //Not the value if necessary
                        let val = if *not { !val } else { val };
                        value_stack.push(val);
                    }
                    //Getting here means that the previous value of history false or didn't match spawn point
                    ExprLTL::Eventually(his) => {
                        //Get value as a boolean
                        let val = value_stack.pop_or_err()?;
                        let bool_val = val.get_value().get_verdict()?;

                        //Update history
                        let (a, b) = bound;
                        let his_idx = (*t_spawn % (*b - *a + 1)) as usize;
                        match his.get_mut(his_idx) {
                            Some(his_val) => {
                                //If Spawn point matches, then only update value
                                if his_val.spawn_point == *t_spawn {
                                    his_val.value = bool_val;
                                //Else update the entire value
                                } else {
                                    *his_val = (bool_val, *t_spawn).into();
                                }
                            }
                            None => {
                                his.resize(his_idx + 1, (false, -1_i128).into());
                                his[his_idx] = (bool_val, *t_spawn).into();
                            }
                        }

                        //If true, then the property has been satisfied and by extension decided
                        // If false and the interval has been active for less time than the bound allows, then undecided
                        value_stack.push(if !bool_val && *t_current < t_upper {
                            val.to_undecided()
                        } else {
                            val
                        })
                    }
                }
            }
            _ => Err(errors::Error::IllegalOperation)?,
        }
    }
    value_stack.pop_or_err()
}

#[inline]
fn function_type_computation(function_type: &AggregateType, cur_val: i128, range: i128) -> i128 {
    match function_type {
        AggregateType::Sum => cur_val,
        AggregateType::Avg => cur_val / (range + 1),
    }
}

///Warning: This function has side effects
#[inline]
fn time_function_reduce_step(
    newest_val: i128,
    t_spawn: i128,
    max_bound: i128,
    history_vec: &mut Vec<HistoryValue<i128>>,
) -> i128 {
    //Which idx should be overwritten
    let arr_idx = (t_spawn % (max_bound + 1)) as usize;

    //Sum up the value according to the history and update history accordingly
    match history_vec.get_mut(arr_idx) {
        Some(HistoryValue { value, spawn_point }) => {
            if *spawn_point == t_spawn {
                *value += newest_val;
            } else {
                *value = newest_val;
                *spawn_point = t_spawn;
            }
            *value
        }
        None => {
            history_vec.resize(arr_idx + 1, (0_i128, -1_i128).into());
            history_vec[arr_idx] = (newest_val, t_spawn).into();
            newest_val
        }
    }
}
