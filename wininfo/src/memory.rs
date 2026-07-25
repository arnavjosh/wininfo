use serde::{Deserialize, Serialize};

/// Information about the system's physical memory.
///
/// All values are measured in bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
    usage_percent: f32,
}

impl MemoryInfo {
    pub(crate) fn new(total_bytes: u64, available_bytes: u64) -> Self {
        let used_bytes = total_bytes - available_bytes;

        Self {
            total_bytes,
            available_bytes,
            used_bytes,
            usage_percent: used_bytes as f32 / total_bytes as f32 * 100.0,
        }
    }

    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn available_bytes(&self) -> u64 {
        self.available_bytes
    }

    pub fn used_bytes(&self) -> u64 {
        self.used_bytes
    }

    pub fn total_gib(&self) -> f64 {
        self.total_bytes as f64 / 1024_f64.powi(3)
    }

    pub fn available_gib(&self) -> f64 {
        self.available_bytes as f64 / 1024_f64.powi(3)
    }

    pub fn used_gib(&self) -> f64 {
        self.used_bytes as f64 / 1024_f64.powi(3)
    }
}
