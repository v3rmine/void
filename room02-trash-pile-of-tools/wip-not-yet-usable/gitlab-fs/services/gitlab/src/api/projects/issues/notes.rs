// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project issue note API endpoints.
//!
//! These endpoints are used for querying project issue notes.

mod create;
mod edit;
mod notes;

pub use self::create::CreateIssueNote;
pub use self::create::CreateIssueNoteBuilder;
pub use self::create::CreateIssueNoteBuilderError;

pub use self::edit::EditIssueNote;
pub use self::edit::EditIssueNoteBuilder;
pub use self::edit::EditIssueNoteBuilderError;

pub use self::notes::IssueNotes;
pub use self::notes::IssueNotesBuilder;
pub use self::notes::IssueNotesBuilderError;
pub use crate::api::helpers::NoteOrderBy;
