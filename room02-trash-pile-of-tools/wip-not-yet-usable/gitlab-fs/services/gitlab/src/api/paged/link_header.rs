// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use http::HeaderMap;
use thiserror::Error;
use url::Url;

use crate::api::PaginationError;

#[derive(Debug)]
struct LinkHeader<'a> {
    url: &'a str,
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> LinkHeader<'a> {
    fn parse(s: &'a str) -> Result<Self, LinkHeaderParseError> {
        let mut parts = s.split(';');

        let url_part = parts.next().expect("a split always has at least one part");
        let url = {
            let part = url_part.trim();
            if part.starts_with('<') && part.ends_with('>') {
                &part[1..part.len() - 1]
            } else {
                return Err(LinkHeaderParseError::NoBrackets);
            }
        };

        let params = parts
            .map(|part| {
                let part = part.trim();
                let mut halves = part.splitn(2, '=');
                let key = halves.next().expect("a split always has at least one part");
                let value = if let Some(value) = halves.next() {
                    if value.starts_with('"') && value.ends_with('"') {
                        &value[1..value.len() - 1]
                    } else {
                        value
                    }
                } else {
                    return Err(LinkHeaderParseError::MissingParamValue);
                };

                Ok((key, value))
            })
            .collect::<Result<Vec<_>, LinkHeaderParseError>>()?;

        Ok(Self { url, params })
    }
}

/// An error which can occur when parsing a link header.
#[derive(Debug, Error)]
pub enum LinkHeaderParseError {
    /// An invalid HTTP header was found.
    #[error("invalid header")]
    InvalidHeader {
        /// The source of the error.
        #[from]
        source: reqwest::header::ToStrError,
    },
    /// The `url` for a `Link` header was missing `<>` brackets.
    #[error("missing brackets around url")]
    NoBrackets,
    /// A parameter for a `Link` header was missing a value.
    #[error("missing parameter value")]
    MissingParamValue,
}

impl LinkHeaderParseError {
    fn invalid_header(source: reqwest::header::ToStrError) -> Self {
        Self::InvalidHeader { source }
    }
}

pub(crate) fn next_page_from_headers(headers: &HeaderMap) -> Result<Option<Url>, PaginationError> {
    let link_headers = headers.get_all(reqwest::header::LINK).iter();
    // GitLab 14.0 will deprecate this header in preference for the W3C spec's `Link` header. Make
    // it less preferred to it in anticipation for this change.
    let links_headers = headers.get_all("Links").iter();
    link_headers
        .chain(links_headers)
        .map(|link| {
            let value = link
                .to_str()
                .map_err(LinkHeaderParseError::invalid_header)?;
            Ok(LinkHeader::parse(value)?)
        })
        .collect::<Result<Vec<_>, PaginationError>>()?
        .into_iter()
        .find_map(|header| {
            let is_next_link = header
                .params
                .into_iter()
                .any(|(key, value)| key == "rel" && value == "next");

            if is_next_link {
                Some(header.url.parse().map_err(PaginationError::from))
            } else {
                None
            }
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use crate::api::paged::link_header::LinkHeader;
    use crate::api::LinkHeaderParseError;

    #[test]
    fn test_link_header_no_brackets() {
        let err = LinkHeader::parse("url; param=value").unwrap_err();
        if let LinkHeaderParseError::NoBrackets = err {
            // expected error
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_link_header_no_param_value() {
        let err = LinkHeader::parse("<url>; param").unwrap_err();
        if let LinkHeaderParseError::MissingParamValue = err {
            // expected error
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_link_header_no_params() {
        let link = LinkHeader::parse("<url>").unwrap();
        assert_eq!(link.url, "url");
        assert_eq!(link.params.len(), 0);
    }

    #[test]
    fn test_link_header_quoted_params() {
        let link = LinkHeader::parse("<url>; param=\"value\"; param2=\"value\"").unwrap();
        assert_eq!(link.url, "url");
        assert_eq!(link.params.len(), 2);
        assert_eq!(link.params[0].0, "param");
        assert_eq!(link.params[0].1, "value");
        assert_eq!(link.params[1].0, "param2");
        assert_eq!(link.params[1].1, "value");
    }

    #[test]
    fn test_link_header_bare_params() {
        let link = LinkHeader::parse("<url>; param=value; param2=value").unwrap();
        assert_eq!(link.url, "url");
        assert_eq!(link.params.len(), 2);
        assert_eq!(link.params[0].0, "param");
        assert_eq!(link.params[0].1, "value");
        assert_eq!(link.params[1].0, "param2");
        assert_eq!(link.params[1].1, "value");
    }
}
