use crate::{
    errors, monitor::{
        streams::{IoTDevice, IoTStream, OutputStream}, types::{StackElement, StackContent, StepType, Verdict},
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
    use StepType::*;

    let mut worklist_stack: Vec<(usize, StepType)> = Vec::with_capacity(50);
    let mut value_stack: Vec<StackContent> = Vec::with_capacity(50);
    let mut device_stack: Vec<StackElement<&IoTDevice>> = Vec::with_capacity(50);
    let mut device_pointer: Option<&IoTDevice> = None;
    let mut time_stack: Vec<i128> = Vec::with_capacity(50);

    worklist_stack.push((0usize, StepType::Deepen));
    time_stack.push(*t_spawn);

    while let Some((cur_idx, step_type)) = worklist_stack.pop() {
        let val = &operations[cur_idx];
        match (val, step_type) {
            (DerivedStream::Number(v), Deepen) => value_stack.push(StackContent::from(*v)),
            (DerivedStream::String(v), Deepen) => value_stack.push(StackContent::String(v)),
            (DerivedStream::Member(member_type), Deepen) => {
                let device =
                    device_pointer.ok_or_else(|| Box::new(errors::Error::DevicePointer))?;
                match member_type {
                    crate::program::member_types::MemberType::Power => {
                        value_stack.push(StackContent::Number(Some(device.power)))
                    }
                    crate::program::member_types::MemberType::Name => {
                        value_stack.push(StackContent::String(&device.name))
                    }
                }
            }
            (DerivedStream::SpawnTime, Deepen) => {
                let offset_time = time_stack.last().ok_or(errors::Error::ArrayMissingValue)?;
                value_stack.push(StackContent::Number(Some(*offset_time)));
            }
            (DerivedStream::Size, Deepen) => value_stack.push(StackContent::Number(Some(
                devices.get_devices(*t_spawn as usize).len() as i128,
            ))),

            (DerivedStream::Sum { .. }, Deepen) => {
                device_stack.push(StackElement::LayerShift);
                let offset_time =
                    *time_stack.last().ok_or(errors::Error::ArrayMissingValue)? as usize;
                device_stack.extend(
                    devices
                        .get_devices(offset_time)
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
                    },
                    StackElement::LayerShift => {
                        device_pointer = None;
                    },
                }
            },
            (DerivedStream::Sumtime { interval_len, idx }, Deepen) => {
                if *t_spawn + *interval_len > *t_current { continue; }

                

                worklist_stack.push((cur_idx, Reduce));
                value_stack.push(0.into());
                value_stack.push(0.into());

            },
            (DerivedStream::Sumtime { interval_len, idx }, Reduce) => todo!(),
            (DerivedStream::Foreach { idx }, Deepen) => todo!(),
            (DerivedStream::Foreach { idx }, Reduce) => todo!(),
            (DerivedStream::Foreach { idx }, ReducePartial) => todo!(),

            (
                DerivedStream::Binary {
                    bin_op,
                    idx_lhs,
                    idx_rhs,
                },
                Deepen,
            ) => todo!(),
            (
                DerivedStream::Binary {
                    bin_op,
                    idx_lhs,
                    idx_rhs,
                },
                Reduce,
            ) => todo!(),
            (
                DerivedStream::Binary {
                    bin_op,
                    idx_lhs,
                    idx_rhs,
                },
                ReducePartial,
            ) => todo!(),
            (DerivedStream::Unary { un_op, idx }, Deepen) => todo!(),
            (DerivedStream::Unary { un_op, idx }, Reduce) => todo!(),
            (DerivedStream::Unary { un_op, idx }, ReducePartial) => todo!(),

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
