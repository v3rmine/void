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

pub async fn content(
    ctx: &AppContext,
    key: &str,
    v: impl ViewRenderer,
    lang: &str,
    contents_metadata: &serde_json::Value,
    prerendered: bool,
) -> Result<Response> {
    if prerendered {
        let cache_key = format!("content:{}-{}", key, lang);
        if let Some(content) = ctx.cache.get::<String>(&cache_key).await? {
            return format::render().html(&content);
        } else {
            let rendered_content = render_content(key, v, lang, contents_metadata)?;
            ctx.cache
                .insert::<String>(&cache_key, &rendered_content)
                .await?;
            return format::render().html(&rendered_content);
        }
    } else {
        format::render().html(&render_content(key, v, lang, contents_metadata)?)
    }
}
