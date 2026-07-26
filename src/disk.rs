use byte_unit::Byte;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    device_id: String,
    size: Option<Byte>,
    free: Option<Byte>,
}

impl DiskInfo {
    pub(crate) fn new(device_id: String, size: Option<Byte>, free: Option<Byte>) -> Self {
        Self {
            device_id,
            size,
            free,
        }
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    pub fn size(&self) -> Option<Byte> {
        self.size
    }
    pub fn free(&self) -> Option<Byte> {
        self.free
    }
}
