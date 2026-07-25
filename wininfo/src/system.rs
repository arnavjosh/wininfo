#[cfg(windows)]
use wmi::{COMLibrary, WMIConnection};

use crate::{cpu::CpuInfo, error::WmrError, memory::MemoryInfo};

pub struct System {
    #[cfg(windows)]
    wmi: WMIConnection,
}

impl System {
    /// Creates a new system information provider
    pub fn new() -> Result<Self, WmrError> {
        #[cfg(windows)]
        {
            let com = COMLibrary::new().map_err(|_| WmrError::Com)?;
            let wmi = WMIConnection::new(com)?;

            Ok(Self { wmi })
        }

        #[cfg(not(windows))]
        {
            Err(WmrError::Unsupported)
        }
    }

    pub fn memory(&self) -> Result<MemoryInfo, WmrError> {
        #[cfg(windows)]
        {
            crate::windows::memory(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WmrError::Unsupported)
        }
    }

    pub fn cpu(&self) -> Result<CpuInfo, WmrError> {
        #[cfg(windows)]
        {
            crate::windows::cpu(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WmrError::Unsupported)
        }
    }
}
