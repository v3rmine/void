use loco_rs::{prelude::ViewRenderer, Error, Result};
use serde::Serialize;
use std::{fs, path::Path};
use tracing::debug;

use crate::{common::AppSettings, utils};

type TeraPostProcessor = std::sync::Arc<dyn Fn(&mut tera::Tera) -> Result<()> + Send + Sync>;

#[derive(Clone)]
pub struct CustomTeraView {
    pub tera: tera::Tera,
    pub tera_post_process: Option<TeraPostProcessor>,

    pub default_context: tera::Context,
    pub settings: AppSettings,
}

impl CustomTeraView {
    pub fn post_process(
        mut self,
        post_process: impl Fn(&mut tera::Tera) -> Result<()> + Send + Sync + 'static,
    ) -> Result<Self> {
        {
            let engine = &mut self.tera;
            post_process(engine)?;
        }

        self.tera_post_process = Some(std::sync::Arc::new(post_process));
        Ok(self)
    }

    fn build_tera_from_settings(settings: &AppSettings) -> Result<tera::Tera> {
        debug!("building Tera instance from settings");
        let path: &Path = settings.views_dir.as_ref();

        if !path.exists() {
            return Err(Error::string(&format!(
                "missing views directory: `{}`",
                path.display()
            )));
        }

        // Initialize Tera with all the files ending in .tera
        let tera_instance = tera::Tera::new(
            path.join("**")
                .join("*.tera")
                .to_str()
                .ok_or_else(|| Error::string("invalid blob"))?,
        )?;

        Ok(Self::add_filters(tera_instance)?)
    }
    pub fn from_settings(settings: &AppSettings) -> Result<Self> {
        let tera = Self::build_tera_from_settings(settings)?;

        let mut ctx = tera::Context::default();

        ctx.insert("author", &settings.author);
        ctx.insert("website_name", &settings.website_name);
        ctx.insert("website_base_url", &settings.website_base_url);

        Ok(Self {
            tera,
            tera_post_process: None,
            default_context: ctx,
            settings: settings.clone(),
        })
    }

    fn add_filters(mut tera_instance: tera::Tera) -> Result<tera::Tera> {
        tera_instance.register_filter("markdown", Self::markdown_filter);

        Ok(tera_instance)
    }

    fn markdown_filter(
        value: &tera::Value,
        _: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let markdown_parser = utils::new_markdown_parser();
        let s = tera::try_get_value!("markdown", "value", String, value);

        let html = markdown_parser.parse(&s).render();

        Ok(tera::Value::String(html))
    }
}

impl ViewRenderer for CustomTeraView {
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String> {
        let mut context = self.default_context.clone();
        context.extend(tera::Context::from_serialize(data)?);

        let is_markdown = key.ends_with(".md");

        let key = if is_markdown {
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

            layout
        } else {
            key.to_string()
        };

        Ok(self.tera.render(&key, &context)?)
    }
}

impl<'a> ViewRenderer for &'a CustomTeraView {
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String> {
        CustomTeraView::render(&self, key, data)
    }
}
