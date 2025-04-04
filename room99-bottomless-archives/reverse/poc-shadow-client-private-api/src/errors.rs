#![allow(deprecated)] // cause()
error_chain! {
    foreign_links {
        Ureq(::ureq::Error);
        Base64Decode(::base64::DecodeError);
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
        Url(::url::ParseError);
    }
}
