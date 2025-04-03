// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project job API endpoints.
//!
//! These endpoints are used for querying CI jobs.

mod cancel;
mod erase;
mod job;
mod jobs;
mod play;
mod retry;
mod trace;

pub use self::cancel::CancelJob;
pub use self::cancel::CancelJobBuilder;
pub use self::cancel::CancelJobBuilderError;

pub use self::erase::EraseJob;
pub use self::erase::EraseJobBuilder;
pub use self::erase::EraseJobBuilderError;

pub use self::job::Job;
pub use self::job::JobBuilder;
pub use self::job::JobBuilderError;

pub use self::jobs::JobScope;
pub use self::jobs::Jobs;
pub use self::jobs::JobsBuilder;
pub use self::jobs::JobsBuilderError;

pub use self::play::JobVariableAttribute;
pub use self::play::JobVariableAttributeBuilder;
pub use self::play::JobVariableAttributeBuilderError;
pub use self::play::PlayJob;
pub use self::play::PlayJobBuilder;
pub use self::play::PlayJobBuilderError;

pub use self::retry::RetryJob;
pub use self::retry::RetryJobBuilder;
pub use self::retry::RetryJobBuilderError;

pub use self::trace::JobTrace;
pub use self::trace::JobTraceBuilder;
pub use self::trace::JobTraceBuilderError;
