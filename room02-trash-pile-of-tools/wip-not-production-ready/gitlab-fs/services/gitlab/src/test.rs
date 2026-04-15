// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "client_api")]
pub mod client;

mod types;
mod webhooks;

#[cfg(feature = "client_api")]
macro_rules! assert_missing_field {
    ($err:expr, $type:tt, $field:expr $(,)?) => {
        let in_err = $err;
        let in_field = $field;
        if let $type::UninitializedField(field) = in_err {
            assert_eq!(field, in_field);
        } else {
            panic!(
                "unexpected error (expected to be missing `{}`): {:?}",
                in_field, in_err,
            );
        }
    };
}
#[cfg(feature = "client_api")]
pub(crate) use assert_missing_field;
