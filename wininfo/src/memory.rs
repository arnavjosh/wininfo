use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
    usage_percent: f32,
}

impl MemoryInfo {
    pub(crate) fn new(total_bytes: u64, available_bytes: u64) -> Self {
        let used_bytes = total_bytes.saturating_sub(available_bytes);

        let usage_percent = if total_bytes == 0 {
            0.0
        } else {
            used_bytes as f32 / total_bytes as f32 * 100.0
        };

        Self {
            total_bytes,
            available_bytes,
            used_bytes,
            usage_percent,
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
