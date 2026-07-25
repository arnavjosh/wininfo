use serde::{Deserialize, Serialize};
//everything is in bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,

    pub available_bytes: u64,

    pub used_bytes: u64,

    pub usage_percent: f32,
}
