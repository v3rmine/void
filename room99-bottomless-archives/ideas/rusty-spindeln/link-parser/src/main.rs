#![forbid(unsafe_code)]
#![deny(
    clippy::complexity,
    clippy::perf,
    clippy::checked_conversions,
    clippy::filter_map_next
)]
#![warn(
    clippy::style,
    clippy::map_unwrap_or,
    clippy::missing_const_for_fn,
    clippy::use_self,
    future_incompatible,
    rust_2018_idioms,
    nonstandard_style
)]
// with configurable values
#![warn(
    clippy::blacklisted_name,
    clippy::cognitive_complexity,
    clippy::disallowed_method,
    clippy::fn_params_excessive_bools,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_repetition_in_bounds,
    clippy::unreadable_literal
)]
#![deny(clippy::wildcard_imports)]
// crate-specific exceptions:
#![allow(dead_code)]

use actix_web::client::Client;

fn main() {
    /* Run actix system and make a first request */
    // let mut actix_system = actix_web::rt::System::new("rusty-spindeln");
    // actix_system.block_on(actix_main());
    let a_tag = "<a target=\"_top\" href=\"/ttest\">hello  world</a>";
    let _html = "<html><head><title>test</title></head><body><a target=\"_blank\" href=\"https://demo.example.org/test?q=foo&bar=n#123\">hello world</a></body></html>";

    xml::parse_a(a_tag);
}

mod xml {
    use std::collections::HashMap;

    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::char,
        error::{context, VerboseError},
        sequence::delimited,
        IResult,
    };

    type NomResult<T, U> = IResult<T, U, VerboseError<T>>;

    pub fn parse_a(_input: &str) {
        let head = "";
    }

    fn xml_head<'a>(input: &'a [u8]) -> NomResult<&[u8], XmlHead<'a>> {
        let starting_tag = char('<');
        let ending_tag = char('>');
        let content; // TODO: Header content
        context("xml_head", delimited(starting_tag, content, ending_tag))(input)
    }

    pub fn xml_foot<'a>(input: &'a [u8]) -> NomResult<&[u8], XmlFoot<'a>> {
        let starting_tag = tag("</");
        let ending_tag = ">";
        context(
            "xml_foot",
            delimited(starting_tag, take_until(ending_tag), tag(ending_tag)),
        )(input)
        .map(|(next_input, res)| (next_input, XmlFoot { tag: res }))
    }

    #[derive(Debug, Default)]
    pub struct XmlBlock<'a> {
        head: XmlHead<'a>,
        content: XmlContent<'a>,
        foot: XmlFoot<'a>,
    }
    #[derive(Debug, Default)]
    pub struct XmlHead<'a> {
        tag: &'a str,
        attrs: HashMap<&'a str, &'a str>,
    }

    #[derive(Debug)]
    pub enum XmlContent<'a> {
        PlainText(&'a str),
        Block(&'a XmlBlock<'a>),
        None,
    }
    impl<'a> Default for XmlContent<'a> {
        fn default() -> Self {
            XmlContent::None
        }
    }

    #[derive(Debug, Default)]
    pub struct XmlFoot<'a> {
        pub tag: &'a [u8],
    }
}

async fn _actix_main() {
    let client = Client::default();

    let _response_body = client
        .get("https://example.org")
        .send()
        .await
        .unwrap()
        .body();
}
