/// Need it to convert, array of type &[_, v] where y < v and x < y using array[x..y] to type &[_, y-x]
use arrayref::array_ref;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct HeadC40<'raw> {
    /// Here it'll always be DC
    pub ident: &'raw [char; 2],
    /// Here it is 01
    pub version: &'raw [char; 2],
    pub certification_authority: &'raw [char; 4],
    pub certification_ident: &'raw [char; 4],
    pub document_issue_date: &'raw [char; 4],
    pub sign_creation_date: &'raw [char; 4],
    pub type_ident: &'raw [char; 2],
}

impl<'raw> From<&'raw [char; 22]> for HeadC40<'raw> {
    fn from(raw: &'raw [char; 22]) -> Self {
        Self {
            ident: array_ref!(raw, 0, 2),
            version: array_ref!(raw, 2, 2),
            certification_authority: array_ref!(raw, 4, 4),
            certification_ident: array_ref!(raw, 8, 4),
            document_issue_date: array_ref!(raw, 12, 4),
            sign_creation_date: array_ref!(raw, 16, 4),
            type_ident: array_ref!(raw, 20, 2),
        }
    }
}

impl<'raw> From<HeadC40<'raw>> for String {
    fn from(head: HeadC40<'raw>) -> Self {
        String::from_iter(
            [
                head.ident.as_slice(),
                head.version.as_slice(),
                head.certification_authority.as_slice(),
                head.certification_ident.as_slice(),
                head.document_issue_date.as_slice(),
                head.sign_creation_date.as_slice(),
                head.type_ident.as_slice(),
            ]
            .concat(),
        )
    }
}
