use loco_rs::prelude::*;

pub fn render_content(
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    contents_metadata: &serde_json::Value,
) -> Result<String> {
    v.render(
        key,
        data!({
          "lang": lang,
          "data": contents_metadata
        }),
    )
}

pub fn content(
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    contents_metadata: &serde_json::Value,
    prerendered: bool,
) -> Result<Response> {
    format::render().html(&render_content(key, v, lang, contents_metadata)?)
}
