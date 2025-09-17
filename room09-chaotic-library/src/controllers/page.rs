use std::collections::HashMap;

use loco_rs::prelude::*;

use crate::{
    common::AppSettings,
    initializers::view_engine::CustomTeraView,
    utils::{self, ContentMetadata},
    views,
};

fn generate_render_page(
    template: &str,
    lang: &str,
    metadata: &serde_json::Value,
    prerendered: bool,
) -> axum::routing::MethodRouter<AppContext> {
    // Clone the passed datas for ownership in the closure
    let template = template.to_owned();
    let lang = lang.to_owned();
    let metadata = metadata.to_owned();
    let prerendered = Box::new(prerendered);

    // Return a GET handler that renders the specified blog page template
    get(|ViewEngine(v): ViewEngine<CustomTeraView>| async move {
        // Here we move to take ownership of the template string
        views::page::page(&template, v, &lang, &metadata, *prerendered)
    })
}

pub fn routes(ctx: &AppContext) -> Routes {
    let settings = AppSettings::from(ctx);
    let mut routes = Routes::new();

    let pages_metadata = utils::list_pages(&settings);
    let content_metadata = utils::list_content(&settings);
    let content_metadata = utils::render_contents(&settings, content_metadata);

    for page in pages_metadata.iter() {
        // Only process Tera html/xml template files
        // TODO: Handle other file types or extensions
        let is_html_tera = page.file_name.ends_with(".html.tera");
        let is_xml_tera = page.file_name.ends_with(".xml.tera");
        if is_html_tera || is_xml_tera {
            // Extract the base name without extension to use as route name
            let route_name = if is_html_tera {
                page.file_name
                    .strip_suffix(".html.tera")
                    .unwrap()
                    .to_string()
            } else {
                format!("{}.xml", page.file_name.strip_suffix(".xml.tera").unwrap())
            };

            // Special case for index: map to root path "/" instead of "/index"
            let route_path = if route_name == "index" {
                "/".to_string()
            } else {
                format!("/{}", route_name)
            };

            let posts_by_language = [
                &[settings.default_language.clone()],
                settings.other_languages.as_slice(),
            ]
            .concat()
            .iter()
            .map(|lang| {
                (
                    lang.clone(),
                    content_metadata
                        .iter()
                        .filter_map(|c| {
                            if c.data.get("page_type") == Some(&data!("post")) && &c.lang == lang {
                                Some(c.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<String, Vec<ContentMetadata>>>();

            // Register the route with our router
            // The template path is relative to the view engine's root
            let template = format!("{}{}", settings.views_pages_subdir, page.path);
            let metadata = data!({
                "this": page.clone(),
                "pages": pages_metadata.clone(),
                "posts": posts_by_language,
                "contents": content_metadata.clone()
            });
            routes = routes.add(
                &route_path,
                generate_render_page(
                    &template,
                    &settings.default_language,
                    &metadata,
                    page.prerendered,
                ),
            );

            for lang in settings.other_languages.iter() {
                routes = routes.add(
                    &format!("/{lang}/{route_path}"),
                    generate_render_page(&template, lang, &metadata, page.prerendered),
                );
            }
        }
    }

    routes
}
