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
mod system;

#[cfg(windows)]
mod windows;

pub use cpu::CpuInfo;
pub use disk::DiskInfo;
pub use error::WmrError;
pub use memory::MemoryInfo;
pub use system::System;

pub type Result<T> = std::result::Result<T, WmrError>;
