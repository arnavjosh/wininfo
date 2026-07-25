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

pub fn memory() -> Result<MemoryInfo, WmrError> {
    let com = COMLibrary::new().map_err(|_| WmrError::Com)?;
    let wmi = WMIConnection::new(com)?;

    let result: Vec<Win32OperatingSystem> = wmi.raw_query(
        "SELECT TotalVisibleMemorySize, FreePhysicalMemory FROM Win32_OperatingSystem",
    )?;
    let os = result.first().ok_or(WmrError::Empty)?;

    // WMI reports KB
    let total = os.total_visible_memory_size * 1024;
    let free = os.free_physical_memory * 1024;
    let used = total - free;

    Ok(MemoryInfo {
        total_bytes: total,
        available_bytes: free,
        used_bytes: used,
        usage_percent: used as f32 / total as f32 * 100.0,
    })
}
