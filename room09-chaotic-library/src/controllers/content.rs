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

    // Return a GET handler that renders the specified blog page template
    get(|ViewEngine(v): ViewEngine<CustomTeraView>| async move {
        // Here we move to take ownership of the template string
        views::content::content(&template, v, &lang, &metadata)
    })
}

pub fn routes(ctx: &AppContext) -> Routes {
    let settings = AppSettings::from(ctx);
    let mut routes = Routes::new();

    let content_metadata = utils::list_content(&settings);

    for content in content_metadata.iter() {
        if content.file_name.ends_with(".md") {
            // Special case for index: map to root path "/" instead of "/index"
            let route_path = format!("{}/{}", content.basename, content.slug);

            let metadata = data!({
              "global": {
                "this": content.clone(),
                "contents": content_metadata.clone()
              }
            });

            // Add language prefix to route path if not default language
            let route_path = if content.lang != settings.default_language {
                format!("{}/{}", content.lang, route_path)
            } else {
                route_path
            };

            // Register the route with our router
            // The template path is relative to the view engine's root
            routes = routes.add(
                &route_path,
                generate_render_content(&format!("{}", content.path), &content.lang, &metadata),
            );
        }
    }

    routes
}
