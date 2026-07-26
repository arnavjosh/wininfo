use serde::{Deserialize, Serialize};
#[cfg(windows)]
use wmi::{COMLibrary, WMIConnection};

use crate::{Result, cpu::CpuInfo, disk::DiskInfo, error::WmrError, memory::MemoryInfo};

pub struct System {
    #[cfg(windows)]
    wmi: WMIConnection,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub disks: Vec<DiskInfo>,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
}

impl System {
    /// Creates a new system information provider
    pub fn new() -> Result<Self> {
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

    pub fn memory(&self) -> Result<MemoryInfo> {
        #[cfg(windows)]
        {
            crate::windows::memory(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WmrError::Unsupported)
        }
    }

    pub fn cpu(&self) -> Result<CpuInfo> {
        #[cfg(windows)]
        {
            crate::windows::cpu(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WmrError::Unsupported)
        }
    }

    pub fn disk(&self) -> Result<Vec<DiskInfo>> {
        #[cfg(windows)]
        {
            crate::windows::disk(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WmrError::Unsupported)
        }
    }

    pub fn info(&self) -> Result<Info> {
        Ok(Info {
            disks: self.disk()?,
            cpu: self.cpu()?,
            memory: self.memory()?,
        })
    }
}
