use maud::{html, Markup};
use typed_builder::TypedBuilder;

use crate::CSS_HASHES;

#[derive(Debug, TypedBuilder)]
pub struct HeadContext {
    #[builder(default = false)]
    pub refresh_every_minute: bool,
}

pub fn Head(context: HeadContext, title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            @if context.refresh_every_minute {
                meta http-equiv="refresh" content="60";
            }
            title { (title) }
            @for (path, (_real_path, hash)) in CSS_HASHES.iter() {
                link rel="stylesheet" href=(format!("/css/{path}")) type="text/css" crossorigin="anonymous" integrity=(hash);
            }
        }
    }
}
