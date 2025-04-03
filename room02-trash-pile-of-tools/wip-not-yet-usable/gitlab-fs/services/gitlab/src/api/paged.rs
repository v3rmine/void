// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod link_header;
mod pagination;

mod all_at_once;
mod lazy;

/// A trait to indicate that an endpoint is pageable.
pub trait Pageable {
    /// Whether the endpoint uses keyset pagination or not.
    fn use_keyset_pagination(&self) -> bool {
        false
    }
}

pub use self::link_header::LinkHeaderParseError;

pub use self::pagination::Pagination;
pub use self::pagination::PaginationError;

pub use self::all_at_once::paged;
pub use self::all_at_once::Paged;

pub use self::lazy::LazilyPagedIter;
