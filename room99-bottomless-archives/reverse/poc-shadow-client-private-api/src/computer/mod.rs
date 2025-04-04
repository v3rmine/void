use url::Url;

mod post_start;
pub use post_start::*;

#[derive(Debug, Clone)]
pub struct ComputerEnv {
  pub url: Option<Url>,
  pub token: Option<String>,
}

default_url_w_token!(ComputerEnv);
