use loco_rs::prelude::*;

use crate::{common::AppSettings, initializers::view_engine::CustomTeraView, utils, views};

fn generate_render_content(
    template: &str,
    lang: &str,
    metadata: &serde_json::Value,
) -> axum::routing::MethodRouter<AppContext> {
    // Clone the passed datas for ownership in the closure
    let template = template.to_owned();
    let lang = lang.to_owned();
    let metadata = metadata.to_owned();
    let prerendered = Box::new(
        metadata["this"]["data"]["prerender"]
            .as_bool()
            .unwrap_or(false),
    );

    // Return a GET handler that renders the specified blog page template
    get(
        |State(ctx): State<AppContext>, ViewEngine(v): ViewEngine<CustomTeraView>| async move {
            // Here we move to take ownership of the template string
            views::content::content(&ctx, &template, v, &lang, &metadata, *prerendered).await
        },
    )
}

pub struct Content {
    pub route: String,
    pub template: String,
    pub lang: String,
    pub metadata: serde_json::Value,
}
pub fn contents(content_metadata: Vec<utils::ContentMetadata>) -> Vec<Content> {
    content_metadata
        .iter()
        .filter_map(|content| {
            if content.file_name.ends_with(".md") {
                let metadata = data!({
                    "this": content.clone(),
                    "contents": content_metadata.clone()
                });

                Some(Content {
                    route: content.route_path.clone(),
                    template: format!("{}", content.path),
                    lang: content.lang.clone(),
                    metadata,
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn routes(ctx: &AppContext) -> Routes {
    let settings = AppSettings::from(ctx);
    let mut routes = Routes::new();

    let content_metadata = utils::list_content(&settings);
    let contents = contents(content_metadata.clone());

    for content in contents.iter() {
        // Register the route with our router
        // The template path is relative to the view engine's root
        routes = routes.add(
            &content.route,
            generate_render_content(&content.template, &content.lang, &content.metadata),
        );
    }

    routes
}
