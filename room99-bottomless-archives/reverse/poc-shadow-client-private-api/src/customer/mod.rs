use url::Url;

#[derive(Debug, Clone)]
pub struct CustomerEnv {
  url: Url,
}

default_url!(CustomerEnv, crate::constants::DEFAULT_CUSTOMER_ENDPOINT);
