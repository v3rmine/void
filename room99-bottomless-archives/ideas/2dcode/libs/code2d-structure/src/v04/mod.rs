use crate::{Binary, Encoded, C40};

pub type V04C40 = C40<String>;
pub type V04Binary = Binary<Vec<u8>>;
pub type V04Encoded = Encoded<String, Vec<u8>>;

mod head_c40;
pub use head_c40::HeadC40;
mod head_binary;
pub use head_binary::HeadBinary;

#[derive(Debug)]
pub struct Code2d<'raw> {
    pub head: Encoded<String, C40<HeadC40<'raw>>>,
    pub c40_message: Option<V04C40>,
    pub bin_message: Option<V04Binary>,
    pub sign: V04Encoded,
    pub appendix: Option<V04Encoded>,
}
