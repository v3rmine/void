use crate::C40;

// NOTE: v02 use the same Head format as v01
pub use crate::v01::HeadC40;

pub type V02C40 = C40<String>;

#[derive(Debug)]
pub struct Code2d<'raw> {
    pub head: C40<HeadC40<'raw>>,
    pub message: V02C40,
    pub sign: V02C40,
}
