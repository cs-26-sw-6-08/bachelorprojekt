use std::error::Error;

use crate::{monitor::{streams::{IoTDevice, IoTStream, OutputStream}, types::{StackContent, Verdict}}, monitor_setup::operation_types::DerivedStream, utils::vec_helper_funcs::ExtVec};

impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) -> Result<(), Box<dyn Error>> {
        for (t_spawn, ver) in self.unresolved_timepoints.iter_mut() {
            *ver = match eval_operations(&mut self.derived_streams, devices, &*t_spawn, &t_current)? {
                Some(0) => Verdict::False,
                Some(_) => Verdict::True,
                None => Verdict::Undecided,
            };
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
) -> Result<Option<i128>, Box<dyn Error>> {
    use StepType::*;

    let mut worklist_stack: Vec<(usize, StepType)> = Vec::with_capacity(50);
    let mut value_stack: Vec<StackContent> = Vec::with_capacity(50);
    let mut device_stack: Vec<DeviceStack> = Vec::with_capacity(50);
    let mut device_pointer: Option<&IoTDevice> = None;
    let mut time_offset_stack: Vec<(i128, i128)> = Vec::with_capacity(50);

    worklist_stack.push((0usize, StepType::Deepen));

    while let Some((cur_idx, step_type)) = worklist_stack.pop() {
        let val = &operations[cur_idx];
        match (val, step_type) {
            (DerivedStream::Number(_), Deepen) => todo!(),
            (DerivedStream::Number(_), Reduce) => todo!(),
            (DerivedStream::Number(_), ReducePartial) => todo!(),
            (DerivedStream::String(_), Deepen) => todo!(),
            (DerivedStream::String(_), Reduce) => todo!(),
            (DerivedStream::String(_), ReducePartial) => todo!(),
            (DerivedStream::Member(member_type), Deepen) => todo!(),
            (DerivedStream::Member(member_type), Reduce) => todo!(),
            (DerivedStream::Member(member_type), ReducePartial) => todo!(),
            (DerivedStream::SpawnTime, Deepen) => todo!(),
            (DerivedStream::SpawnTime, Reduce) => todo!(),
            (DerivedStream::SpawnTime, ReducePartial) => todo!(),
            (DerivedStream::Size, Deepen) => todo!(),
            (DerivedStream::Size, Reduce) => todo!(),
            (DerivedStream::Size, ReducePartial) => todo!(),

            (DerivedStream::Sum { idx }, Deepen) => todo!(),
            (DerivedStream::Sum { idx }, Reduce) => todo!(),
            (DerivedStream::Sum { idx }, ReducePartial) => todo!(),
            (DerivedStream::Sumtime { interval_len, idx }, Deepen) => todo!(),
            (DerivedStream::Sumtime { interval_len, idx }, Reduce) => todo!(),
            (DerivedStream::Sumtime { interval_len, idx }, ReducePartial) => todo!(),
            (DerivedStream::Foreach { idx }, Deepen) => todo!(),
            (DerivedStream::Foreach { idx }, Reduce) => todo!(),
            (DerivedStream::Foreach { idx }, ReducePartial) => todo!(),

            (DerivedStream::Binary { bin_op, idx_lhs, idx_rhs }, Deepen) => todo!(),
            (DerivedStream::Binary { bin_op, idx_lhs, idx_rhs }, Reduce) => todo!(),
            (DerivedStream::Binary { bin_op, idx_lhs, idx_rhs }, ReducePartial) => todo!(),
            (DerivedStream::Unary { un_op, idx }, Deepen) => todo!(),
            (DerivedStream::Unary { un_op, idx }, Reduce) => todo!(),
            (DerivedStream::Unary { un_op, idx }, ReducePartial) => todo!(),

            (DerivedStream::Miitl { miitl_type, bound, idx }, Deepen) => todo!(),
            (DerivedStream::Miitl { miitl_type, bound, idx }, Reduce) => todo!(),
            (DerivedStream::Miitl { miitl_type, bound, idx }, ReducePartial) => todo!(),
        }
    }

    value_stack.pop_or_err()?.get_num()
}
