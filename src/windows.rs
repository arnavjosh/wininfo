use byte_unit::Byte;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use wmi::WMIConnection;

use crate::{
    Result, cpu::CpuInfo, disk::DiskInfo, error::WinInfoError, memory::MemoryInfo,
    network::NetworkAdapterInfo,
};

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

    let os = result.first().ok_or(WinInfoError::Empty)?;

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

    let processor = result.first().ok_or(WinInfoError::Empty)?;

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

#[derive(Debug, Deserialize, Serialize)]
struct Win32LogicalDisk {
    #[serde(rename = "DeviceID")]
    device_id: Option<String>,

    #[serde(rename = "Size")]
    size: Option<u64>,

    #[serde(rename = "FreeSpace")]
    free: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Win32NetworkAdapter {
    #[serde(rename = "Name")]
    name: Option<String>,

    #[serde(rename = "MACAddress")]
    mac_address: Option<String>,

    #[serde(rename = "NetEnabled")]
    net_enabled: Option<bool>,

    #[serde(rename = "Speed")]
    speed: Option<u64>,

    #[serde(rename = "InterfaceIndex")]
    interface_index: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct Win32NetworkAdapterConfiguration {
    #[serde(rename = "InterfaceIndex")]
    interface_index: Option<u32>,

    #[serde(rename = "IPAddress")]
    ip_address: Option<Vec<String>>,
}

pub(crate) fn disk(wmi: &WMIConnection) -> Result<Vec<DiskInfo>> {
    let result: Vec<Win32LogicalDisk> = wmi.raw_query(
        "SELECT DeviceID, Size, FreeSpace \
         FROM Win32_LogicalDisk",
    )?;

    let mut all_disks: Vec<DiskInfo> = Vec::with_capacity(result.len());

    for disk in &result {
        let info = DiskInfo::new(
            disk.device_id
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            disk.size.map(Byte::from_u64),
            disk.free.map(Byte::from_u64),
        );
        all_disks.push(info);
    }

    Ok(all_disks)
}

pub(crate) fn network_adapters(wmi: &WMIConnection) -> Result<Vec<NetworkAdapterInfo>> {
    let adapters: Vec<Win32NetworkAdapter> = wmi.raw_query(
        "SELECT Name, MACAddress, NetEnabled, Speed, InterfaceIndex \
         FROM Win32_NetworkAdapter",
    )?;

    let configurations: Vec<Win32NetworkAdapterConfiguration> = wmi.raw_query(
        "SELECT InterfaceIndex, IPAddress \
         FROM Win32_NetworkAdapterConfiguration \
         WHERE IPEnabled = TRUE",
    )?;

    let ipv4_by_interface_index: HashMap<u32, Ipv4Addr> = configurations
        .into_iter()
        .filter_map(|configuration| {
            let interface_index = configuration.interface_index?;
            let ipv4_address = configuration.ip_address.and_then(|addresses| {
                addresses
                    .into_iter()
                    .find_map(|address| address.parse::<Ipv4Addr>().ok())
            });

            ipv4_address.map(|ipv4_address| (interface_index, ipv4_address))
        })
        .collect();

    let mut all_adapters = Vec::with_capacity(adapters.len());

    for adapter in adapters {
        let name = adapter.name.unwrap_or_else(|| "Unknown".to_string());
        let ipv4_address = adapter
            .interface_index
            .and_then(|index| ipv4_by_interface_index.get(&index).copied());
        all_adapters.push(NetworkAdapterInfo::new(
            name,
            adapter.mac_address,
            ipv4_address,
            adapter.net_enabled.unwrap_or(false),
            adapter.speed,
        ));
    }

    Ok(all_adapters)
}

#[derive(Debug, Deserialize)]
struct UWFOverlay {
    #[serde(rename = "OverlayConsumption")]
    overlay_consumption: u32,

    #[serde(rename = "AvailableSpace")]
    available_space: u32,
}

#[derive(Debug, Deserialize)]
struct UWFFilter {
    #[serde(rename = "CurrentEnabled")]
    current_enabled: bool,
}
