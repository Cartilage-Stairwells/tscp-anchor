use serde::Serialize;
use crate::types::TransitionError;

pub fn to_canonical_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>, TransitionError> {
    serde_cbor::to_vec(value)
       .map_err(|e| TransitionError::Serialization(e.to_string()))
}

pub fn from_cbor<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, TransitionError> {
    serde_cbor::from_slice(bytes)
       .map_err(|e| TransitionError::Serialization(e.to_string()))
}
