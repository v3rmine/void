// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request note API endpoints.
//!
//! These endpoints are used for querying project merge request notes.

pub mod awards;
mod create;
mod edit;
mod notes;

pub use self::create::CreateMergeRequestNote;
pub use self::create::CreateMergeRequestNoteBuilder;
pub use self::create::CreateMergeRequestNoteBuilderError;

pub use self::edit::EditMergeRequestNote;
pub use self::edit::EditMergeRequestNoteBuilder;
pub use self::edit::EditMergeRequestNoteBuilderError;

pub use self::notes::MergeRequestNotes;
pub use self::notes::MergeRequestNotesBuilder;
pub use self::notes::MergeRequestNotesBuilderError;
pub use crate::api::helpers::NoteOrderBy;
