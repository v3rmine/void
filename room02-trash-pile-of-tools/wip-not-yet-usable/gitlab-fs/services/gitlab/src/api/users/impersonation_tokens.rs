// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Impersonation token endpoints
//!
//! All of these endpoints are admin-only.

mod create;
mod delete;
mod impersonation_token;
mod impersonation_tokens;

pub use self::create::CreateImpersonationToken;
pub use self::create::CreateImpersonationTokenBuilder;
pub use self::create::CreateImpersonationTokenBuilderError;
pub use self::create::ImpersonationTokenScope;

pub use self::delete::DeleteImpersonationToken;
pub use self::delete::DeleteImpersonationTokenBuilder;
pub use self::delete::DeleteImpersonationTokenBuilderError;

pub use self::impersonation_token::ImpersonationToken;
pub use self::impersonation_token::ImpersonationTokenBuilder;
pub use self::impersonation_token::ImpersonationTokenBuilderError;

pub use self::impersonation_tokens::ImpersonationTokenState;
pub use self::impersonation_tokens::ImpersonationTokens;
pub use self::impersonation_tokens::ImpersonationTokensBuilder;
pub use self::impersonation_tokens::ImpersonationTokensBuilderError;
