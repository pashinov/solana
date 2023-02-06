//! Errors related to proving and verifying range proofs.
use {
    crate::errors::{ProofVerificationError, TranscriptError},
    thiserror::Error,
};

#[derive(Error, Clone, Debug)]
#[error("range proof verification failed: {0}")]
pub struct RangeProofError(#[from] pub(crate) ProofVerificationError);
impl_from_transcript_error!(RangeProofError);
impl_from_infallible_error!(RangeProofError);
