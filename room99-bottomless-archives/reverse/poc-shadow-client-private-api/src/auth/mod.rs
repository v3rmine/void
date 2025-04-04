use url::Url;

mod post_auth_login;
pub use post_auth_login::*;

#[derive(Debug, Clone)]
pub struct AuthEnv {
  pub url: Url,
}

default_url!(AuthEnv, crate::constants::DEFAULT_AUTH_ENDPOINT);
