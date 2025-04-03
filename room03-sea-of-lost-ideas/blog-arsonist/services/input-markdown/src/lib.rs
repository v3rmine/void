#![feature(generic_associated_types)]
use async_trait::async_trait;
use process_trait::ProcessTrait;
use pulldown_cmark::{html::push_html, Options, Parser};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

#[derive(Debug)]
pub struct Markdown;

#[async_trait]
impl ProcessTrait for Markdown {
    type Input<'a> = &'a str;
    type Output<'a> = String;
    type Error = Error;

    #[tracing::instrument]
    async fn process<'input, 'output>(
        input: Self::Input<'input>,
    ) -> Result<Self::Output<'static>, Self::Error> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(input, options);
        let mut html_output = String::new();
        push_html(&mut html_output, parser);

        Ok(html_output)
    }
}
