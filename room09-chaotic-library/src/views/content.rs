use loco_rs::prelude::*;

pub fn content(
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    contents_metadata: &serde_json::Value,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        key,
        data!({
          "lang": lang,
          "data": contents_metadata
        }),
    )
}
