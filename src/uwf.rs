use byte_unit::Byte;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UWFInfo {
    enabled: bool,
    total_size: Option<Byte>,
    used_size: Option<Byte>,
}

impl UWFInfo {
    pub(crate) fn new(enabled: bool, total_size: Option<Byte>, used_size: Option<Byte>) -> Self {
        Self {
            enabled,
            total_size,
            used_size,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn total_size(&self) -> Option<Byte> {
        self.total_size
    }
    pub fn used_size(&self) -> Option<Byte> {
        self.used_size
    }
}
