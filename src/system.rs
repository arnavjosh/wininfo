use serde::{Deserialize, Serialize};
#[cfg(windows)]
use wmi::{COMLibrary, WMIConnection};

use crate::{
    Result,
    cpu::CpuInfo,
    disk::DiskInfo,
    error::WinInfoError,
    memory::MemoryInfo,
    network::NetworkAdapterInfo,
};

pub struct System {
    #[cfg(windows)]
    wmi: WMIConnection,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub disks: Vec<DiskInfo>,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub network_adapters: Vec<NetworkAdapterInfo>,
}

impl System {
    /// Creates a new system information provider
    pub fn new() -> Result<Self> {
        #[cfg(windows)]
        {
            let com = COMLibrary::new().map_err(|_| WinInfoError::Com)?;
            let wmi = WMIConnection::new(com)?;

            Ok(Self { wmi })
        }

        #[cfg(not(windows))]
        {
            Err(WinInfoError::Unsupported)
        }
    }

    pub fn memory(&self) -> Result<MemoryInfo> {
        #[cfg(windows)]
        {
            crate::windows::memory(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WinInfoError::Unsupported)
        }
    }

    pub fn cpu(&self) -> Result<CpuInfo> {
        #[cfg(windows)]
        {
            crate::windows::cpu(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WinInfoError::Unsupported)
        }
    }

    pub fn disks(&self) -> Result<Vec<DiskInfo>> {
        #[cfg(windows)]
        {
            crate::windows::disk(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WinInfoError::Unsupported)
        }
    }

    pub fn network_adapters(&self) -> Result<Vec<NetworkAdapterInfo>> {
        #[cfg(windows)]
        {
            crate::windows::network_adapters(&self.wmi)
        }

        #[cfg(not(windows))]
        {
            Err(WinInfoError::Unsupported)
        }
    }

    pub fn info(&self) -> Result<Info> {
        Ok(Info {
            disks: self.disks()?,
            cpu: self.cpu()?,
            memory: self.memory()?,
            network_adapters: self.network_adapters()?,
        })
    }
}
