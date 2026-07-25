use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    name: String,
    manufacturer: String,
    physical_cores: Option<u32>,
    logical_cores: Option<u32>,
    max_clock_speed_mhz: Option<u64>,
}

impl CpuInfo {
    pub(crate) fn new(
        name: String,
        manufacturer: String,
        physical_cores: Option<u32>,
        logical_cores: Option<u32>,
        max_clock_speed_mhz: Option<u64>,
    ) -> Self {
        Self {
            name,
            manufacturer,
            physical_cores,
            logical_cores,
            max_clock_speed_mhz,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }
    pub fn physical_cores(&self) -> Option<u32> {
        self.physical_cores
    }
    pub fn logical_cores(&self) -> Option<u32> {
        self.logical_cores
    }
    pub fn max_clock_speed_mhz(&self) -> Option<u64> {
        self.max_clock_speed_mhz
    }
    pub fn max_clock_speed_ghz(&self) -> Option<f64> {
        self.max_clock_speed_mhz.map(|mhz| mhz as f64 / 1000.0)
    }
}
