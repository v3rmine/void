use crate::{Binary, C40};

pub type V01Binary = Binary<Vec<u8>>;

mod head_c40;
pub use head_c40::HeadC40;
mod message_c40;
pub use message_c40::MessageC40;
mod message_parts_c40;

#[derive(Debug)]
pub struct Code2d<'raw> {
    pub head: C40<HeadC40<'raw>>,
    pub message: C40<MessageC40>,
    pub sign: V01Binary,
}
