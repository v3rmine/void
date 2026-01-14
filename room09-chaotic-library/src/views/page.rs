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
    v.render(
        key,
        data!({
          "lang": lang,
          "data": pages_metadata
        }),
    )
}

pub async fn page(
    ctx: &AppContext,
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    pages_metadata: &serde_json::Value,
    prerendered: bool,
) -> Result<Response> {
    let content = if prerendered {
        let cache_key = format!("page:{}-{}", key, lang);
        if let Some(content) = ctx.cache.get::<String>(&cache_key).await? {
            content
        } else {
            dbg!("Rendering content");
            let rendered_content = render_page(key, v, lang, pages_metadata)?;
            ctx.cache
                .insert::<String>(&cache_key, &rendered_content)
                .await?;
            rendered_content
        }
    } else {
        render_page(key, v, lang, pages_metadata)?
    };

    // dbg!(&pages_metadata);
    dbg!(&key);
    if key.ends_with(".xml.tera") || key.ends_with(".xml") {
        Ok(Builder::new()
            .header(header::CONTENT_TYPE, "text/xml")
            .body(Body::from(content))?
            .into_response())
    } else {
        format::html(&content)
    }
}
