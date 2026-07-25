mod error;
mod memory;

#[cfg(windows)]
mod windows;

pub use error::WmrError;
pub use memory::MemoryInfo;

pub fn memory() -> Result<MemoryInfo, WmrError> {
    #[cfg(windows)]
    {
        windows::memory()
    }

    #[cfg(not(windows))]
    {
        Err(WmrError::Unsupported)
    }
}
