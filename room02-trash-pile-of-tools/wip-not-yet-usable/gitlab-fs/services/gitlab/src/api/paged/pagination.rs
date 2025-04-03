// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

use crate::api::paged::LinkHeaderParseError;

/// Errors which may occur with pagination.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PaginationError {
    /// A `Link` HTTP header can fail to parse.
    #[error("failed to parse a Link HTTP header: {}", source)]
    LinkHeader {
        /// The source of the error.
        #[from]
        source: LinkHeaderParseError,
    },
    /// An invalid URL can be returned.
    #[error("failed to parse a Link HTTP header URL: {}", source)]
    InvalidUrl {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
}

/// Pagination options for GitLab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pagination {
    /// Return all results.
    ///
    /// Note that some endpoints may have a server-side limit to the number of results (e.g.,
    /// `/projects` is limited to 10000 results).
    #[default]
    All,
    /// Limit to a number of results.
    Limit(usize),
}

const MAX_PAGE_SIZE: usize = 100;

impl Pagination {
    pub(crate) fn page_limit(self) -> usize {
        match self {
            Pagination::All => MAX_PAGE_SIZE,
            Pagination::Limit(size) => size.min(MAX_PAGE_SIZE),
        }
    }

    pub(crate) fn is_last_page(self, last_page_size: usize, num_results: usize) -> bool {
        // If the last page has fewer elements than our limit, we're definitely done.
        if last_page_size < self.page_limit() {
            return true;
        }

        // Otherwise, check if we have results which fill our limit.
        if let Pagination::Limit(limit) = self {
            return limit <= num_results;
        }

        // We're not done yet.
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Pagination;

    #[test]
    fn pagination_default() {
        assert_eq!(Pagination::default(), Pagination::All);
    }
}
