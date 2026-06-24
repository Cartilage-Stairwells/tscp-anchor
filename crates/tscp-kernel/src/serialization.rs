use crate::types::TransitionError;
use serde::{Deserialize, Serialize};

pub fn to_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>, TransitionError> {
    serde_cbor::to_vec(value).map_err(|e| TransitionError::PreconditionFailed(e.to_string()))
}

pub fn from_cbor<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, TransitionError> {
    serde_cbor::from_slice(bytes).map_err(|e| TransitionError::PreconditionFailed(e.to_string()))
}
