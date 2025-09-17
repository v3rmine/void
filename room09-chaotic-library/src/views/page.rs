use axum::{
    body::Body,
    http::{header, response::Builder},
};
use loco_rs::prelude::*;

pub fn render_page(
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    pages_metadata: &serde_json::Value,
) -> Result<String> {
    // dbg!(&pages_metadata);
    v.render(
        key,
        data!({
          "lang": lang,
          "data": pages_metadata
        }),
    )
}

pub fn page(
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    pages_metadata: &serde_json::Value,
    prerendered: bool,
) -> Result<Response> {
    dbg!(&pages_metadata);
    let rendered_page = render_page(key, v, lang, pages_metadata)?;
    dbg!(&key);
    if key.ends_with(".xml.tera") || key.ends_with(".xml") {
        Ok(Builder::new()
            .header(header::CONTENT_TYPE, "text/xml")
            .body(Body::from(rendered_page))?
            .into_response())
    } else {
        format::html(&rendered_page)
    }
}
