use thiserror::Error;

#[derive(Error, Debug)]
pub enum WmrError {
    #[cfg(windows)]
    #[error("WMI error")]
    Wmi(#[from] wmi::WMIError),
    #[error("COM initialization problemo")]
    Com,

    #[error("Didnt get results bruh.")]
    Empty,

    #[error("platform wont work")]
    Unsupported,
}
