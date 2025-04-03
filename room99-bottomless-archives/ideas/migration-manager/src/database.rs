use crate::Result;

#[tracing::instrument]
fn get_dialect_from_url(url: &str) -> Result<String> {
    let dialect = url
        .split("://")
        .next()
        .ok_or_else(|| eyre::eyre!("Invalid database url"))?;
    Ok(dialect.to_string())
}
