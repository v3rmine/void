use loco_rs::prelude::*;

use crate::{common::AppSettings, initializers::view_engine::CustomTeraView, utils, views};

fn generate_render_page(
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
        views::page::page(&template, v, &lang, &metadata)
    })
}

pub fn routes(ctx: &AppContext) -> Routes {
    let settings = AppSettings::from(ctx);
    let mut routes = Routes::new();

    let pages_metadata = utils::list_pages(&settings);
    let content_metadata = utils::list_content(&settings);

    for page in pages_metadata.iter() {
        // Only process Tera html template files
        // TODO: Handle other file types or extensions
        if page.file_name.ends_with(".html.tera") {
            // Extract the base name without extension to use as route name
            let route_name = page.file_name.strip_suffix(".html.tera").unwrap();

            // Special case for index: map to root path "/" instead of "/index"
            let route_path = if route_name == "index" {
                "/".to_string()
            } else {
                format!("/{}", route_name)
            };

            // Register the route with our router
            // The template path is relative to the view engine's root
            let template = format!("blog/{}", page.file_name);
            let metadata = data!({
              "global": {
                "this": page.clone(),
                "pages": pages_metadata.clone(),
                "contents": content_metadata.clone()
              }
            });
            routes = routes.add(
                &route_path,
                generate_render_page(&template, &settings.default_language, &metadata),
            );

            for lang in settings.other_languages.iter() {
                routes = routes.add(
                    &format!("/{lang}/{route_path}"),
                    generate_render_page(&template, lang, &metadata),
                );
            }
        }
    }

    routes
}
