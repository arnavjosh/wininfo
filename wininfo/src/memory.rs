use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
}

impl MemoryInfo {
    pub(crate) fn new(total_bytes: u64, available_bytes: u64) -> Self {
        let used_bytes = total_bytes.saturating_sub(available_bytes);

        Self {
            total_bytes,
            available_bytes,
            used_bytes,
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
        bytes_to_gib(self.total_bytes)
    }
    pub fn available_gib(&self) -> f64 {
        bytes_to_gib(self.available_bytes)
    }
    pub fn used_gib(&self) -> f64 {
        bytes_to_gib(self.used_bytes)
    }
}

fn bytes_to_gib(bytes: u64) -> f64 {
    bytes as f64 / 1024_f64.powi(3)
}
