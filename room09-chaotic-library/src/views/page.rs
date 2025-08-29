use loco_rs::prelude::*;

pub fn page(
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    pages_metadata: &serde_json::Value,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        key,
        data!({
          "lang": lang,
          "data": pages_metadata
        }),
    )
}
