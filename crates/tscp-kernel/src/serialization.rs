use crate::types::TransitionError;
use serde::{Deserialize, Serialize};

pub fn to_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>, TransitionError> {
    let mut vec = Vec::new();
    ciborium::ser::into_writer(value, &mut vec)
        .map_err(|e| TransitionError::PreconditionFailed(e.to_string()))?;
    Ok(vec)
}

pub fn from_cbor<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, TransitionError> {
    ciborium::de::from_reader(bytes).map_err(|e| TransitionError::PreconditionFailed(e.to_string()))
}
