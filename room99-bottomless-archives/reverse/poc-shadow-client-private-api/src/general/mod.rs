use url::Url;

#[derive(Debug, Clone)]
pub struct GeneralEnv {
  url: Url,
  token: Option<String>,
}

default_url_w_token!(GeneralEnv, crate::constants::DEFAULT_GENERAL_ENDPOINT);
