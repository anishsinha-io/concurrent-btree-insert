use serde::{de::DeserializeOwned, Serialize};

///
/// This function encodes an object of a generic type T into a vector of bytes, as long as the type T implements the Sized and Serialize
/// traits
///
pub fn encode<T>(item: T) -> Option<Vec<u8>>
where
    T: Sized + Serialize,
{
    let encoded = bincode::serialize(&item);
    match encoded {
        Ok(encoded) => Some(encoded),
        Err(_) => None,
    }
}

///
/// This function tries to decodes a vector of bytes into an object of generic type T, as long as the type T implements the Sized,
/// Serialize, and DeserializeOwned traits
///
pub fn decode<T>(bytes: Vec<u8>) -> Option<T>
where
    T: Sized + Serialize + DeserializeOwned,
{
    let decoded = bincode::deserialize(&bytes[..]);
    match decoded {
        Ok(decoded) => Some(decoded),
        Err(_) => None,
    }
}
