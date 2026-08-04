use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapterInfo {
    name: String,
    mac_address: Option<String>,
    ipv4_address: Option<Ipv4Addr>,
    enabled: bool,
    speed: Option<u64>,
}

impl NetworkAdapterInfo {
    pub(crate) fn new(
        name: String,
        mac_address: Option<String>,
        ipv4_address: Option<Ipv4Addr>,
        enabled: bool,
        speed: Option<u64>,
    ) -> Self {
        Self {
            name,
            mac_address,
            ipv4_address,
            enabled,
            speed,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mac_address(&self) -> Option<&str> {
        self.mac_address.as_deref()
    }

    pub fn ipv4_address(&self) -> Option<Ipv4Addr> {
        self.ipv4_address
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn speed(&self) -> Option<u64> {
        self.speed
    }
}