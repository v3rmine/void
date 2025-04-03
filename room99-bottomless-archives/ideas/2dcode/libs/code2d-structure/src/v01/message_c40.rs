use super::message_parts_c40;

#[derive(Debug)]
pub struct MessageC40(Vec<MessagePartC40>);

pub enum MessagePartC40 {
    Text,
    Numeric,
    Date,
    Time,
}

/*pub trait MessagePart {
    fn get_raw_identifier(&self) -> &'_ [char; 2];
    fn get_identifier(&self);
    fn is_required_part(&self) -> bool;
    fn get_encoding(&self);
}*/
