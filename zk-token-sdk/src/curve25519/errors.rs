use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum Curve25519Error {
    #[error("pod conversion failed")]
    PodConversion,
    #[error(transparent)]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}
