use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Mutex};
use tracing::{error, info};

use crate::{
    common::AppSettings, controllers, initializers::view_engine::CustomTeraView, utils, views,
};

pub struct LinkCheckerWorker {
    pub ctx: AppContext,
    pub checked_links: Mutex<HashSet<String>>,
}

impl LinkCheckerWorker {
    pub async fn check(&self, html: &str, template: &str, lang: &str) {
        let document = scraper::Html::parse_document(&html);
        let link_selector = scraper::Selector::parse(r#"a[href^="http"]"#).unwrap();

        for link in document.select(&link_selector).into_iter() {
            let Some(href) = link.value().attr("href") else {
                continue;
            };

            {
                let href = href.to_string();
                let mut checked_links = self.checked_links.lock().unwrap();
                if checked_links.contains(&href) {
                    continue;
                } else {
                    checked_links.insert(href);
                }
            }

            if let Ok(res) = ureq::get(href).call() {
                if res.status().is_client_error() || res.status().is_server_error() {
                    error!(link = href, template, lang, "link check failed");
                } else {
                    info!(link = href, template, lang, "link check passed");
                }
            } else {
                error!(link = href, template, lang, "link check failed");
            }
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LinkCheckerWorkerArgs {}

#[async_trait]
impl BackgroundWorker<LinkCheckerWorkerArgs> for LinkCheckerWorker {
    fn build(ctx: &AppContext) -> Self {
        Self {
            ctx: ctx.clone(),
            checked_links: Mutex::new(HashSet::new()),
        }
    }
    async fn perform(&self, _args: LinkCheckerWorkerArgs) -> Result<()> {
        let settings = AppSettings::from(&self.ctx);
        let tera_view_engine = CustomTeraView::from_settings(&settings).unwrap();

        let content_metadata = utils::list_content(&settings);
        let contents = controllers::content::contents(content_metadata);
        for content in contents.iter() {
            let check_links = content.metadata["this"]["data"]["check_links"]
                .as_bool()
                .unwrap_or(false);

            if check_links {
                let html = views::content::render_content(
                    &content.template,
                    tera_view_engine.clone(),
                    &content.lang,
                    &content.metadata,
                )
                .unwrap();

                self.check(&html, &content.template, &content.lang).await;
            }
        }

        // let page_metadata = utils::list_pages(&settings);

        Ok(())
    }
}
