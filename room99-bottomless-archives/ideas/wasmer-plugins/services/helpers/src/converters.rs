use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

/// Convert binary to Rust Type (*which implements [`Deserialize`]*) using [`deserialize`]
///
/// By Free Masen <https://github.com/FreeMasen/wasmer-plugin/blob/master/src/lib.rs>
///
/// [`bincode`]: https://docs.rs/bincode
pub fn convert_slice<'i, D>(slice: &'i [u8]) -> D
where
    D: Deserialize<'i>,
{
    match deserialize(slice) {
        Ok(ret) => ret,
        Err(e) => {
            panic!("error deserializing {}", e);
        }
    }
}
/// Convert a Rust Type to binary (*which implements [`Serialize`]*) using [`serialize`]
///
/// By Free Masen <https://github.com/FreeMasen/wasmer-plugin/blob/master/src/lib.rs>
///
/// [`bincode`]: https://docs.rs/bincode
pub fn convert_ret<S>(ret: S) -> Vec<u8>
where
    S: Serialize,
{
    match serialize(&ret) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("error serializing {}", e)
        }
    }
}
