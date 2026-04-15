// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project container registry API endpoints.
//!
//! These endpoints are used for querying the container registry.

mod delete_repository;
mod delete_repository_tag;
mod repositories;
mod repository_tag_details;
mod repository_tags;

pub use self::delete_repository::DeleteRepository;
pub use self::delete_repository::DeleteRepositoryBuilder;
pub use self::delete_repository::DeleteRepositoryBuilderError;

pub use self::delete_repository_tag::DeleteRepositoryTag;
pub use self::delete_repository_tag::DeleteRepositoryTagBuilder;
pub use self::delete_repository_tag::DeleteRepositoryTagBuilderError;

pub use self::repositories::Repositories;
pub use self::repositories::RepositoriesBuilder;
pub use self::repositories::RepositoriesBuilderError;

pub use self::repository_tag_details::RepositoryTagDetails;
pub use self::repository_tag_details::RepositoryTagDetailsBuilder;
pub use self::repository_tag_details::RepositoryTagDetailsBuilderError;

pub use self::repository_tags::RepositoryTags;
pub use self::repository_tags::RepositoryTagsBuilder;
pub use self::repository_tags::RepositoryTagsBuilderError;
