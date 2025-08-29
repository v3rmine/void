#![feature(generic_associated_types)]
use async_trait::async_trait;
use gray_matter::{engine::YAML, Matter};
use process_trait::{Frontmatter, ProcessTrait};
use thiserror::Error;
use tracing::trace;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing YAML frontmatter: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct YamlFrontmatter;

#[async_trait]
impl ProcessTrait for YamlFrontmatter {
    type Input<'a> = &'a str;
    type Output<'a> = (String, Option<Frontmatter>);
    type Error = Error;

    #[tracing::instrument]
    async fn process<'input, 'output>(
        input: Self::Input<'input>,
    ) -> Result<Self::Output<'static>, Self::Error> {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(input);
        trace!(content = result.content, data = ?result.data);
        let serialiazed = result.data.map(|data| data.deserialize());

        if serialiazed.is_some() {
            Ok((
                result.content,
                serialiazed.unwrap().map_err(Error::ParseError)?,
            ))
        } else {
            Ok((result.content, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use process_trait::ProcessTrait;

    use crate::YamlFrontmatter;

    #[assay::assay]
    async fn parse_simple_yaml() {
        let front = YamlFrontmatter::process(
            r#"---
title: yaml-frontmatter
tags:
    - gray-matter
    - rust
---
Content"#,
        )
        .await?;
        assert_eq!("Content", front.0);
        assert_eq!("yaml-frontmatter", front.1.unwrap().title.unwrap());
    }
}
