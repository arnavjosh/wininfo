use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

use crate::{error::WmrError, memory::MemoryInfo};

#[derive(Debug, Deserialize)]
struct Win32OperatingSystem {
    #[serde(rename = "TotalVisibleMemorySize")]
    total_visible_memory_size: u64,

    #[serde(rename = "FreePhysicalMemory")]
    free_physical_memory: u64,
}

pub(crate) fn memory() -> Result<MemoryInfo, WmrError> {
    let com = COMLibrary::new().map_err(|_| WmrError::Com)?;
    let wmi = WMIConnection::new(com)?;

    let result: Vec<Win32OperatingSystem> = wmi.raw_query(
        "SELECT TotalVisibleMemorySize, FreePhysicalMemory FROM Win32_OperatingSystem",
    )?;

    let os = result.first().ok_or(WmrError::Empty)?;

    // WMI reports memory values in KiB
    let total = os.total_visible_memory_size * 1024;
    let available = os.free_physical_memory * 1024;

    Ok(MemoryInfo::new(total, available))
}
