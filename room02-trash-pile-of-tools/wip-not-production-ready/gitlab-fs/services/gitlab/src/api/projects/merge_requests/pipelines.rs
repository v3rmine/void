// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Handles merge request pipeline API endpoints.
//!
//! These endpoints are used for handling merge request pipelines.

mod create;
mod pipelines;

pub use self::create::CreateMergeRequestPipelines;
pub use self::create::CreateMergeRequestPipelinesBuilder;
pub use self::create::CreateMergeRequestPipelinesBuilderError;

pub use self::pipelines::MergeRequestPipelines;
pub use self::pipelines::MergeRequestPipelinesBuilder;
pub use self::pipelines::MergeRequestPipelinesBuilderError;
