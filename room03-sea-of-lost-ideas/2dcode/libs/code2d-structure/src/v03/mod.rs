use crate::C40;

pub type V03C40 = C40<String>;

mod head_c40;
pub use head_c40::HeadC40;

#[derive(Debug)]
pub struct Code2d<'raw> {
    pub head: C40<HeadC40<'raw>>,
    pub message: V03C40,
    pub sign: V03C40,
}
