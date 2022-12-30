use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum WasiCommonError {
    #[error("FdMap is full")]
    FdMapFull,
    #[error("{0}")]
    StringArray(StringArrayError),
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum StringArrayError {
    #[error("Number of elements exceeds 2^32")]
    NumberElements,
    #[error("Element size exceeds 2^32")]
    ElementSize,
    #[error("Cumulative size exceeds 2^32")]
    CumulativeSize,
}
