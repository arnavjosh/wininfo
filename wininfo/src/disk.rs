use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    device_id: String,
    size: Option<u64>,
    free_space: Option<u64>,
}

impl DiskInfo {
    pub(crate) fn new(device_id: String, size: Option<u64>, free_space: Option<u64>) -> Self {
        Self {
            device_id,
            size,
            free_space,
        }
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    pub fn size(&self) -> Option<u64> {
        self.size
    }
    pub fn free_space(&self) -> Option<u64> {
        self.free_space
    }
}
