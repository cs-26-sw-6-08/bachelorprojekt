use crate::{errors, monitor::types::Verdict, monitor_setup::operation_types::DerivedStream};

#[derive(Debug, PartialEq)]
pub struct OutputStream {
    pub(crate) unresolved_timepoints: Vec<(i128, Verdict)>,
    pub(crate) derived_streams: Vec<DerivedStream>,
}

impl From<Vec<DerivedStream>> for OutputStream {
    fn from(value: Vec<DerivedStream>) -> Self {
        Self {
            derived_streams: value,
            unresolved_timepoints: Vec::new(),
        }
    }
}

impl OutputStream {
    pub fn get_operations(&self) -> &Vec<DerivedStream> {
        &self.derived_streams
    }

    /// Insert a time point into the output stream.
    pub fn insert(&mut self, t: i128) {
        self.unresolved_timepoints.push((t, Verdict::Undecided))
    }

    /// Gives verdict to the user based on the time_verdicts.
    pub fn get_verdict_mul(&self) -> Vec<i128> {
        self.unresolved_timepoints
            .iter()
            .filter_map(|(time, verdict)| (*verdict == Verdict::False).then_some(*time))
            .collect()
    }

    /// Having True returned means violation
    pub fn get_violated_verdict_single(&mut self) -> bool {
        self.unresolved_timepoints
            .iter()
            .any(|(_, verdict)| *verdict == Verdict::False)
    }

    /// Cleans up time_verdicts.
    pub fn clean_up(&mut self) {
        self.unresolved_timepoints
            .retain(|(_, verdict)| *verdict == Verdict::Undecided);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IoTDevice {
    pub name: String,
    pub power: i128,
}

impl From<(String, i128)> for IoTDevice {
    fn from(value: (String, i128)) -> Self {
        let (mut name, power) = value;
        name = name.to_lowercase();
        Self { name, power }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IoTStream(Vec<Vec<IoTDevice>>);
impl IoTStream {
    pub fn get_devices(&self, i: usize) -> &Vec<IoTDevice> {
        &self.0[i]
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn get_mut_devices(&mut self, i: usize) -> &mut Vec<IoTDevice> {
        &mut self.0[i]
    }

    pub fn get_devices_own(&self, i: usize) -> Vec<IoTDevice> {
        self.0[i].clone()
    }

    pub fn get_all_own(self) -> Vec<Vec<IoTDevice>> {
        self.0
    }

    pub fn push_at(&mut self, devices: Vec<IoTDevice>, i: usize) -> Result<(), errors::Error> {
        self.0
            .get_mut(i)
            .map(|to_set| *to_set = devices)
            .ok_or(errors::Error::OutOfBoundsIoTStream)?;
        Ok(())
    }
}

impl From<Vec<Vec<IoTDevice>>> for IoTStream {
    fn from(value: Vec<Vec<IoTDevice>>) -> Self {
        Self(value)
    }
}

impl From<Vec<IoTDevice>> for IoTStream {
    fn from(value: Vec<IoTDevice>) -> Self {
        Self(vec![value])
    }
}

