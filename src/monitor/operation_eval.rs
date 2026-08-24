use crate::{
    errors, monitor::{
        streams::{IoTDevice, IoTStream, OutputStream}, types::{StackElement, StepType, StreamValue, Verdict},
    }, monitor_setup::operation_types::DerivedStream, utils::vec_helper_funcs::ExtVec,
};
use std::error::Error;

impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) -> Result<(), Box<dyn Error>> {
        for (t_spawn, ver) in self.unresolved_timepoints.iter_mut() {
            *ver = match eval_operations(&mut self.derived_streams, devices, &*t_spawn, &t_current)?
            {
                Some(0) => Verdict::False,
                Some(_) => Verdict::True,
                None => Verdict::Undecided,
            };
        }
        Ok(())
    }
}

pub(crate) fn eval_operations(
    operations: &mut [DerivedStream],
    devices: &IoTStream,
    t_spawn: &i128,
    t_current: &i128,
    //todo: Should this return verdict ???
) -> Result<Option<i128>, Box<dyn Error>> {
    use StackElement::*;
    use StepType::*;

    let mut worklist_stack: Vec<(usize, StepType)> = Vec::with_capacity(50);
    let mut value_stack: Vec<StreamValue> = Vec::with_capacity(50);
    let mut device_stack: Vec<StackElement<&IoTDevice>> = Vec::with_capacity(50);
    let mut device_pointer: Option<&IoTDevice> = None;
    let mut time_stack: Vec<StackElement<i128>> = Vec::with_capacity(50);
    let mut t_offset: i128 = *t_spawn;

    worklist_stack.push((0usize, StepType::Deepen));
    time_stack.push(StackElement::Element(*t_spawn));

    while let Some((cur_idx, step_type)) = worklist_stack.pop() {
        let val = &operations[cur_idx];
        match (val, step_type) {
            (DerivedStream::Number(v), Deepen) => value_stack.push(StreamValue::from(*v)),
            (DerivedStream::String(v), Deepen) => value_stack.push(StreamValue::String(v)),
            (DerivedStream::Member(member_type), Deepen) => {
                let device =
                    device_pointer.ok_or_else(|| Box::new(errors::Error::DevicePointer))?;
                match member_type {
                    crate::program::member_types::MemberType::Power => {
                        value_stack.push(StreamValue::Number(Some(device.power)))
                    }
                    crate::program::member_types::MemberType::Name => {
                        value_stack.push(StreamValue::String(&device.name))
                    }
                }
            }
            (DerivedStream::SpawnTime, Deepen) => {
                //let offset_time = time_stack.last().ok_or(errors::Error::ArrayMissingValue)?;
                value_stack.push(StreamValue::Number(Some(t_offset)));
            }
            (DerivedStream::Size, Deepen) => value_stack.push(StreamValue::Number(Some(
                devices.get_devices(*t_spawn as usize).len() as i128,
            ))),

            (DerivedStream::Sum { .. }, Deepen) => {
                device_stack.push(StackElement::LayerShift);
                // let offset_time = time_stack.last_or_err()?.unpack_element()?;

                device_stack.extend(
                    devices
                        .get_devices(t_offset as usize)
                        .iter()
                        .map(StackElement::Element),
                );

                worklist_stack.push((cur_idx, Reduce));
                value_stack.push(0.into());
                value_stack.push(0.into());
            }
            (DerivedStream::Sum { idx }, Reduce) => {
                let res = value_stack.pop_or_err()? + value_stack.pop_or_err()?;
                value_stack.push(res);

                match device_stack.pop_or_err()? {
                    StackElement::Element(iot_device) => {
                        worklist_stack.push((cur_idx, Reduce));
                        worklist_stack.push((*idx, Deepen));
                        device_pointer = Some(iot_device);
                    }
                    StackElement::LayerShift => {
                        device_pointer = None;
                    }
                }
            }
            (DerivedStream::Sumtime { interval_len, idx }, Deepen) => {
                //let t_offset = *time_stack.last_or_err()?.unpack_element()?;
                if *t_current >= t_offset + *interval_len {
                    value_stack.push(StreamValue::Number(None));
                    continue;
                }

                time_stack.push(Element(t_offset));
                time_stack.push(LayerShift);
                time_stack.extend(
                    (t_offset..t_offset + *interval_len)
                        .into_iter()
                        .map(Element),
                );

                worklist_stack.push((cur_idx, Reduce));
                value_stack.push(0.into());
                value_stack.push(0.into());
            }
            (DerivedStream::Sumtime { interval_len, idx }, Reduce) => {
                let res = value_stack.pop_or_err()? + value_stack.pop_or_err()?;
                value_stack.push(res);

                match time_stack.pop_or_err()? {
                    Element(v) => {
                        worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                        value_stack.push(0.into());
                        t_offset = v;
                    }
                    LayerShift => {
                        t_offset = *time_stack.pop_or_err()?.unpack_element()?;
                    }
                }
            }
            (DerivedStream::Foreach { .. }, Deepen) => {
                worklist_stack.push((cur_idx, Reduce));
                device_stack.push(StackElement::LayerShift);
                for d in devices.get_devices(t_offset as usize) {
                    device_stack.push(d.into());
                }
                value_stack.push(StreamValue::Number(Some(true as i128)))
            }
            (DerivedStream::Foreach { idx }, Reduce) => {
                //Violation didn't occur and not all devices have been looked at
                if value_stack
                        .last()
                        .is_some_and(|v| matches!(v.get_verdict(), Ok(true)))
                    && device_stack
                        .last()
                        .is_some_and(|v| matches!(v, StackElement::Element(_)))
                {
                    let _ = value_stack.pop_or_err()?;
                    device_pointer = match device_stack.pop() {
                        Some(StackElement::Element(v)) => Some(v),
                        Some(StackElement::LayerShift) | None => unreachable!(),
                    };
                    worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                } else {
                    while let Some(StackElement::Element(_)) = device_stack.pop() {}
                }
            }

            // BinOp / UnOp
            (DerivedStream::Binary { idx_lhs, .. }, Deepen) => {
                worklist_stack.extend([(cur_idx, ReducePartial), (*idx_lhs, Deepen)]);
            }
            (
                DerivedStream::Binary {
                    idx_rhs, ..
                },
                ReducePartial,
            ) => {
                worklist_stack.extend([(cur_idx, Reduce), (*idx_rhs, Deepen)]);
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

            (
                DerivedStream::Miitl {
                    miitl_type,
                    bound,
                    idx,
                },
                Deepen,
            ) => todo!(),
            (
                DerivedStream::Miitl {
                    miitl_type,
                    bound,
                    idx,
                },
                Reduce,
            ) => todo!(),
            (
                DerivedStream::Miitl {
                    miitl_type,
                    bound,
                    idx,
                },
                ReducePartial,
            ) => todo!(),

            _ => unreachable!(),
        }
    }

    value_stack.pop_or_err()?.get_num()
}
