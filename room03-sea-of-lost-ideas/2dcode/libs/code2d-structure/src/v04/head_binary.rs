/// Need it to convert, array of type &[_, v] where y < v and x < y using array[x..y] to type &[_, y-x]
use arrayref::array_ref;

#[derive(Debug)]
pub struct HeadBinary<'raw> {
    /// Here it'll always be 0xDC
    pub ident: &'raw [u8; 1],
    /// Here it is 0x04
    pub version: &'raw [u8; 1],
    /// Using the format ISO-3166-Alpha3
    pub issuer_country: &'raw [u8; 2],
    pub certification_and_sign_ident: &'raw [u8; 6],
    pub document_issue_date: &'raw [u8; 3],
    pub sign_creation_date: &'raw [u8; 3],
    pub type_ident: &'raw [u8; 1],
    pub perimeter_ident: &'raw [u8; 2],
}

impl<'raw> From<&'raw [u8; 19]> for HeadBinary<'raw> {
    fn from(raw: &'raw [u8; 19]) -> Self {
        Self {
            ident: array_ref!(raw, 0, 1),
            version: array_ref!(raw, 1, 1),
            issuer_country: array_ref!(raw, 2, 2),
            certification_and_sign_ident: array_ref!(raw, 4, 6),
            document_issue_date: array_ref!(raw, 10, 3),
            sign_creation_date: array_ref!(raw, 13, 3),
            type_ident: array_ref!(raw, 16, 1),
            perimeter_ident: array_ref!(raw, 17, 2),
        }
    }
}

impl<'raw> From<HeadBinary<'raw>> for Vec<u8> {
    fn from(head: HeadBinary<'raw>) -> Self {
        [
            head.ident.as_slice(),
            head.version.as_slice(),
            head.issuer_country.as_slice(),
            head.certification_and_sign_ident.as_slice(),
            head.document_issue_date.as_slice(),
            head.sign_creation_date.as_slice(),
            head.type_ident.as_slice(),
            head.perimeter_ident.as_slice(),
        ]
        .concat()
    }
}
