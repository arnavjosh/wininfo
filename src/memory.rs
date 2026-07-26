use byte_unit::Byte;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    total: Byte,
    free: Byte,
}

impl MemoryInfo {
    pub(crate) fn new(total: Byte, free: Byte) -> Self {
        Self { total, free }
    }
    pub fn total(&self) -> Byte {
        self.total
    }
    pub fn free(&self) -> Byte {
        self.free
    }
}
