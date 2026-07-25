use byte_unit::Byte;
use serde::Deserialize;
use wmi::WMIConnection;

use crate::{Result, cpu::CpuInfo, disk::DiskInfo, error::WmrError, memory::MemoryInfo};

#[derive(Debug, Deserialize)]
struct Win32OperatingSystem {
    #[serde(rename = "TotalVisibleMemorySize")]
    total_visible_memory_size: u64,

    #[serde(rename = "FreePhysicalMemory")]
    free_physical_memory: u64,
}

#[derive(Debug, Deserialize)]
struct Win32Processor {
    #[serde(rename = "Name")]
    name: Option<String>,

    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,

    #[serde(rename = "NumberOfCores")]
    number_of_cores: Option<u32>,

    #[serde(rename = "NumberOfLogicalProcessors")]
    number_of_logical_processors: Option<u32>,

    #[serde(rename = "MaxClockSpeed")]
    max_clock_speed: Option<u64>,
}

pub(crate) fn memory(wmi: &WMIConnection) -> Result<MemoryInfo> {
    let result: Vec<Win32OperatingSystem> = wmi.raw_query(
        "SELECT TotalVisibleMemorySize, FreePhysicalMemory \
         FROM Win32_OperatingSystem",
    )?;

    let os = result.first().ok_or(WmrError::Empty)?;

    // WMI reports these values in KiB
    let total = Byte::from_u64(os.total_visible_memory_size * 1024);
    let free = Byte::from_u64(os.free_physical_memory * 1024);

    Ok(MemoryInfo::new(total, free))
}

pub(crate) fn cpu(wmi: &WMIConnection) -> Result<CpuInfo> {
    let result: Vec<Win32Processor> = wmi.raw_query(
        "SELECT Name, Manufacturer, NumberOfCores, \
         NumberOfLogicalProcessors, MaxClockSpeed \
         FROM Win32_Processor",
    )?;

    let processor = result.first().ok_or(WmrError::Empty)?;

    Ok(CpuInfo::new(
        processor
            .name
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
        processor
            .manufacturer
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
        processor.number_of_cores,
        processor.number_of_logical_processors,
        processor.max_clock_speed,
    ))
}

#[derive(Debug, Deserialize)]
struct Win32LogicalDisk {
    #[serde(rename = "DeviceID")]
    device_id: Option<String>,

    #[serde(rename = "Size")]
    size: Option<u64>,

    #[serde(rename = "FreeSpace")]
    free_space: Option<u64>,
}

pub(crate) fn disk(wmi: &WMIConnection) -> Result<DiskInfo> {
    let result: Vec<Win32LogicalDisk> = wmi.raw_query(
        "SELECT DeviceID, Size, FreeSpace \
         FROM Win32_LogicalDisk",
    )?;

    let disk = result.first().ok_or(WmrError::Empty)?;

    Ok(DiskInfo::new(
        disk.device_id
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
        disk.size,
        disk.free_space,
    ))
}
