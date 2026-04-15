use maud::{html, Markup, DOCTYPE};
use typed_builder::TypedBuilder;

use crate::{components::Head, HeadContext, WEBSITE_TITLE};

#[derive(Debug, Default, TypedBuilder)]
pub struct BaseContext {
    #[builder(default = false)]
    pub refresh_every_minute: bool,
}

pub fn Base(context: BaseContext, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (Head(HeadContext::builder().refresh_every_minute(context.refresh_every_minute).build(), WEBSITE_TITLE))
            body {
                main .container { (children) }
            }
        }
    }
}
