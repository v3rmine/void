#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LoadAvg {
    /// Number of jobs in the run queue averaged over 1 minute.
    pub one: f64,
    /// Number of jobs in the run queue averaged over 5 minute.
    pub five: f64,
    /// Number of jobs in the run queue averaged over 15 minute.
    pub fifteen: f64,
}

impl LoadAvg {
    pub fn new() -> Result<Self> {
        let psutil_load_avg = psutil::host::loadavg()?;
        Ok(Self {
            one: psutil_load_avg.one,
            five: psutil_load_avg.five,
            fifteen: psutil_load_avg.fifteen,
        })
    }
}
