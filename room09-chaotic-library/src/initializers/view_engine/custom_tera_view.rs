use loco_rs::{prelude::ViewRenderer, Error, Result};
use serde::Serialize;
#[cfg(debug_assertions)]
use std::sync::{Arc, Mutex};
use std::{fs, path::Path};

use crate::common::AppSettings;

type TeraPostProcessor = std::sync::Arc<dyn Fn(&mut tera::Tera) -> Result<()> + Send + Sync>;

#[derive(Clone)]
pub struct CustomTeraView {
    #[cfg(debug_assertions)]
    pub tera: std::sync::Arc<std::sync::Mutex<tera::Tera>>,
    #[cfg(not(debug_assertions))]
    pub tera: tera::Tera,
    pub tera_post_process: Option<TeraPostProcessor>,

    pub default_context: tera::Context,
    pub settings: AppSettings,
}

impl CustomTeraView {
    pub fn build(settings: &AppSettings) -> Result<Self> {
        let view_engine = Self::from_settings(settings)?;

        // Register a filter to convert markdown to HTML
        #[cfg(debug_assertions)]
        {
            let mut tera = view_engine.tera.lock().unwrap();
            tera.register_filter("markdown", Self::markdown_filter);
        }

        #[cfg(not(debug_assertions))]
        view_engine
            .tera
            .register_filter("markdown", Self::markdown_filter);

        Ok(view_engine)
    }

    pub fn post_process(
        mut self,
        post_process: impl Fn(&mut tera::Tera) -> Result<()> + Send + Sync + 'static,
    ) -> Result<Self> {
        {
            #[cfg(debug_assertions)]
            let engine = &mut *self.tera.lock().unwrap();

            #[cfg(not(debug_assertions))]
            let engine = &mut self.tera;

            post_process(engine)?;
        }

        self.tera_post_process = Some(std::sync::Arc::new(post_process));
        Ok(self)
    }

    pub fn tera_from_settings(settings: &AppSettings) -> Result<tera::Tera> {
        let path: &Path = settings.views_dir.as_ref();

        if !path.exists() {
            return Err(Error::string(&format!(
                "missing views directory: `{}`",
                path.display()
            )));
        }

        // Initialize Tera with all the files ending in .tera
        Ok(tera::Tera::new(
            path.join("**")
                .join("*.tera")
                .to_str()
                .ok_or_else(|| Error::string("invalid blob"))?,
        )?)
    }
    pub fn from_settings(settings: &AppSettings) -> Result<Self> {
        let tera = Self::tera_from_settings(settings)?;

        #[cfg(debug_assertions)]
        let tera = Arc::new(Mutex::new(tera));

        let ctx = tera::Context::default();
        Ok(Self {
            tera,
            tera_post_process: None,
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

            dbg!(&context);

            layout
        } else {
            key.to_string()
        };

        #[cfg(debug_assertions)]
        {
            tracing::debug!(key = key, "Tera rendering in non-optimized debug mode");
            let mut tera = Self::tera_from_settings(&self.settings)?;
            if let Some(post_process) = self.tera_post_process.as_deref() {
                post_process(&mut tera)?;
            }
            Ok(tera.render(&key, &context)?)
        }

        #[cfg(not(debug_assertions))]
        Ok(self.tera.render(&key, &context)?)
    }
}
