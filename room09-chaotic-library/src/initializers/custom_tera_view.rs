use loco_rs::{prelude::ViewRenderer, Error, Result};
use serde::Serialize;
use std::{fs, path::Path};

use crate::common::AppSettings;

#[derive(Clone, Debug)]
pub struct CustomTeraView {
    pub tera: tera::Tera,
    pub default_context: tera::Context,
    pub settings: AppSettings,
}

impl CustomTeraView {
    pub fn build(settings: &AppSettings) -> Result<Self> {
        let mut view_engine = Self::from_settings(settings)?;

        // Register a filter to convert markdown to HTML
        view_engine
            .tera
            .register_filter("markdown", Self::markdown_filter);

        Ok(view_engine)
    }

    pub fn from_settings(settings: &AppSettings) -> Result<Self> {
        let path: &Path = settings.views_dir.as_ref();

        if !path.exists() {
            return Err(Error::string(&format!(
                "missing views directory: `{}`",
                path.display()
            )));
        }

        // Initialize Tera with all the files ending in .tera
        let tera = tera::Tera::new(
            path.join("**")
                .join("*.tera")
                .to_str()
                .ok_or_else(|| Error::string("invalid blob"))?,
        )?;

        let ctx = tera::Context::default();
        Ok(Self {
            tera,
            default_context: ctx,
            settings: settings.clone(),
        })
    }

    fn markdown_filter(
        value: &tera::Value,
        _: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let mut markdown_parser = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut markdown_parser);
        markdown_it::plugins::extra::add(&mut markdown_parser);

        let s = tera::try_get_value!("markdown", "value", String, value);

        let html = markdown_parser.parse(&s).render();

        Ok(tera::Value::String(html))
    }
}

impl ViewRenderer for CustomTeraView {
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String> {
        let mut context = tera::Context::from_serialize(data)?;

        let is_markdown = key.ends_with(".md");

        if is_markdown {
            // Use the default layout unless specified
            let layout = context
                .get("layout")
                .and_then(|layout| layout.as_str())
                .unwrap_or(&self.settings.default_markdown_layout)
                .to_string();

            let content = fs::read_to_string(format!("{}{key}", self.settings.contents_dir))
                .unwrap_or_default();
            // Content without frontmatter
            let content_without_frontmatter = content.splitn(3, "---").last().unwrap_or_default();

            context.insert(
                "content",
                &tera::Value::String(content_without_frontmatter.to_string()),
            );

            dbg!(&context);

            Ok(self.tera.render(&layout, &context)?)
        } else {
            Ok(self.tera.render(&key, &context)?)
        }
    }
}
