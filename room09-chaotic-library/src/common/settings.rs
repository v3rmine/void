use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AppSettings {
    #[serde(default = "default::language")]
    pub default_language: String,
    #[serde(default)]
    pub other_languages: Vec<String>,
    #[serde(default = "default::views_dir")]
    pub views_dir: String,
    #[serde(default = "default::markdown_layout")]
    pub default_markdown_layout: String,
    #[serde(default = "default::i18n_dir")]
    pub i18n_dir: String,
    #[serde(default = "default::i18n_shared")]
    pub i18n_shared: String,
    #[serde(default = "default::contents_dir")]
    pub contents_dir: String,
    #[serde(default = "default::views_pages_subdir")]
    pub views_pages_subdir: String,
    #[serde(default = "default::website_name")]
    pub website_name: String,
    #[serde(default = "default::website_base_url")]
    pub website_base_url: String,
    #[serde(default = "default::author")]
    pub author: String,
}

mod default {
    pub fn language() -> String {
        "fr".to_string()
    }
    pub fn views_dir() -> String {
        "assets/views".to_string()
    }
    pub fn markdown_layout() -> String {
        "_layouts/default.html.tera".to_string()
    }
    pub fn i18n_dir() -> String {
        "assets/i18n".to_string()
    }
    pub fn i18n_shared() -> String {
        "assets/i18n/shared.ftl".to_string()
    }
    pub fn contents_dir() -> String {
        "contents".to_string()
    }
    pub fn views_pages_subdir() -> String {
        "pages".to_string()
    }
    pub fn website_name() -> String {
        "".to_string()
    }
    pub fn website_base_url() -> String {
        "https://example.com".to_string()
    }
    pub fn author() -> String {
        "".to_string()
    }
}

impl AppSettings {
    pub fn from_json(value: &serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(value.clone())?)
    }
}

impl From<&AppContext> for AppSettings {
    fn from(ctx: &AppContext) -> Self {
        ctx.config
            .settings
            .as_ref()
            .and_then(|settings| Self::from_json(settings).ok())
            .unwrap_or_else(|| Self::default())
    }
}
