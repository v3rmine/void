pub type PInput<'input> = crate::Span<'input>;

pub use symbols::*;
mod symbols {
    // Lines
    pub const LINE_HEADER_SYMBOL: &str = "*";

    // Parts
    pub const BOLD_SURROUNDING_SYMBOL: &str = "*";
    pub const ITALIC_SURROUNDING_SYMBOL: &str = "*";
    pub const STRIKETHROUGH_SURROUNDING_SYMBOL: &str = "~";
    pub const UNDERLINE_SURROUNDING_SYMBOL: &str = "_";
    pub const CODE_SURROUNDING_SYMBOL: &str = "~";
    pub const VERBATIM_SURROUNDING_SYMBOL: &str = "=";

    pub const SUPERSCRIPT_START_SYMBOL: &str = "^";
    pub const SUPERSCRIPT_SURROUNDING_SYMBOL_START: &str = "{";
    pub const SUPERSCRIPT_SURROUNDING_SYMBOL_END: &str = "}";

    pub const SUBSCRIPT_START_SYMBOL: &str = "_";
    pub const SUBSCRIPT_SURROUNDING_SYMBOL_START: &str = "{";
    pub const SUBSCRIPT_SURROUNDING_SYMBOL_END: &str = "}";

    pub const CHECKBOX_SURROUNDING_SYMBOL_START: &str = "[";
    pub const CHECKBOX_CHECKED_SYMBOL: &str = "x";
    pub const CHECKBOX_UNCHECKED_SYMBOL: &str = " ";
    pub const CHECKBOX_SURROUNDING_SYMBOL_END: &str = "]";

    pub const COUNTER_SURROUNDING_SYMBOL_START: &str = "[";
    pub const COUNTER_SEPARATOR_SYMBOL: &str = "/";
    pub const COUNTER_SURROUNDING_SYMBOL_END: &str = "]";

    pub const LINK_SURROUNDING_SYMBOL_START: &str = "[";
    pub const LINK_TARGET_SURROUNDING_SYMBOL_START: &str = "[";
    pub const LINK_TARGET_SURROUNDING_SYMBOL_END: &str = "]";
    pub const LINK_DESCRIPTION_SURROUNDING_SYMBOL_START: &str = "[";
    pub const LINK_DESCRITPION_SURROUNDING_SYMBOL_END: &str = "]";
    pub const LINK_SURROUNDING_SYMBOL_END: &str = "]";
}

pub use block::*;
mod block {}

pub use line::*;
mod line {
    use crate::{PInput, PartsType};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum LinesType<'input> {
        Header(Header<'input>),
        Plain(Vec<PartsType<'input>>),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Header<'input> {
        pub level: u8,
        pub title: PInput<'input>,
        pub parts: Vec<PartsType<'input>>,
    }
}

pub use part::*;
mod part {
    use crate::PInput;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PartsType<'input> {
        Superscript(Superscript<'input>),
        Subscript(Subscript<'input>),
        Bold(Bold<'input>),
        Italic(Italic<'input>),
        StrikeThrough(Strikethrough<'input>),
        Underline(Underline<'input>),
        Code(Code<'input>),
        Verbatim(Verbatim<'input>),
        Counter(Counter),
        Checkbox(Checkbox),
        Link(Link<'input>),
        Plain(PInput<'input>),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Bold<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Italic<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Strikethrough<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Underline<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Code<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Verbatim<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Superscript<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Subscript<'input>(pub PInput<'input>);

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Counter {
        pub current: u32,
        pub total: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Checkbox(pub bool);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Link<'input> {
        pub target: PInput<'input>,
        pub desc: Option<PInput<'input>>,
    }
}
