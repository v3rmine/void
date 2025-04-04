use base64;
use serde::Deserialize;
use url::Url;
use urlencoding;

use crate::{client::InnerResponse, Response, Shadow, ToResp};

#[derive(Debug, Clone)]
pub struct DispatcherEnv {
  pub url: Url,
}

default_url!(DispatcherEnv, crate::constants::DEFAULT_DISPATCHER_ENDPOINT);

#[derive(Deserialize, Debug, Clone)]
pub struct DispatcherResponse {
  pub uri: String,
}

impl Shadow {
  /*pub fn dispatch(&self) -> Response<DispatcherResponse> {
    let resp = ureq::get(&format!(
      "{}gap?email={}&fmt=json",
      self.inner.dispatcher.url,
      urlencoding::encode(&base64::encode(&self.email))
    ))
    .set("Authentification", "Bearer undefined");
  }*/
}
