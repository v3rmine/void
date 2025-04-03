// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Hook structures
//!
//! Hooks are received from Gitlab when registered as a system or web hook listener.
//!
//! Gitlab does not have consistent structures for its hooks, so they often change from
//! version to version.

use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};
use serde_json::{self, Value};

use crate::systemhooks::SystemHook;
use crate::webhooks::WebHook;

/// A deserializable structure for all Gitlab hooks.
#[derive(Debug, Clone)]
pub enum GitlabHook {
    /// A system hook.
    System(SystemHook),
    /// A web hook from a specific project.
    Web(Box<WebHook>),
}

impl<'de> Deserialize<'de> for GitlabHook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = <Value as Deserialize>::deserialize(deserializer)?;

        // Look for `object_kind` first because some web hooks also have `event_name` which would
        // cause a false match here.
        let hook_res = if val.pointer("/object_kind").is_some() {
            serde_json::from_value(val).map(|hook| GitlabHook::Web(Box::new(hook)))
        } else if val.pointer("/event_name").is_some() {
            serde_json::from_value(val).map(GitlabHook::System)
        } else {
            return Err(D::Error::missing_field("either object_kind or event_name"));
        };

        hook_res.map_err(|err| {
            D::Error::invalid_value(
                Unexpected::Other("gitlab hook"),
                &format!("{:?}", err).as_str(),
            )
        })
    }
}
