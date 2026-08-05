#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::assertions_on_result_states
    )
)]

mod cpu;
mod disk;
mod error;
mod memory;
mod network;
mod system;
mod uwf;

#[cfg(windows)]
mod windows;

pub use byte_unit::Byte;
pub use cpu::CpuInfo;
pub use disk::DiskInfo;
pub use error::WinInfoError;
pub use memory::MemoryInfo;
pub use network::NetworkAdapterInfo;
pub use system::System;
pub use uwf::UWFInfo;

pub type Result<T> = std::result::Result<T, WinInfoError>;
