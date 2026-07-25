use thiserror::Error;

#[derive(Debug, Error)]
pub enum WmrError {
    #[cfg(windows)]
    #[error("WMI error: {0}")]
    Wmi(#[from] wmi::WMIError),

    #[error("COM initialization failed")]
    Com,

    #[error("No results returned from WMI")]
    Empty,

    #[error("Unsupported platform")]
    Unsupported,
}
