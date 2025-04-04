use base64;
use serde::Deserialize;
use urlencoding;

use crate::constants::{
  DEFAULT_ACCEPT_ENCODING, DEFAULT_ACCEPT_LANGUAGE, DEFAULT_SHADOW_AGENT, DEFAULT_USER_AGENT,
};
use crate::{client::InnerResponse, Response, Shadow, ToResp};

#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResponse {
  pub token: String,
}

impl Shadow {
  pub fn auth_login(&self) -> Response<AuthLoginResponse> {
    // TODO self.inner.computer.url
    let resp: InnerResponse<String> = ureq::post(&format!(
      "{}/shadow/auth_login",
      "https://prod.gap.pa1.blade-group.fr:2443/"
    ))
    .set("Connection", "keep-alive")
    .set("User-Agent", DEFAULT_USER_AGENT)
    .set("X-Shadow-Uuid", &self.shadow_uuid)
    .set("X-Shadow-Agent", DEFAULT_SHADOW_AGENT)
    .set("Sec-Fetch-Site", "none")
    .set("Sec-Fetch-Mode", "no-cors")
    .set("Accept-Encoding", DEFAULT_ACCEPT_ENCODING)
    .set("Accept-Language", DEFAULT_ACCEPT_LANGUAGE)
    .set("Content-Type", "application/x-www-form-urlencoded")
    .send_string(&format!(
      "email={}&pwd64={}",
      urlencoding::encode(&self.email),
      urlencoding::encode(&base64::encode(&self.password))
    ))
    .to_response()?;

    Ok(InnerResponse {
      headers: resp.headers,
      status_code: resp.status_code,
      status: resp.status,
      raw_response: serde_json::from_str::<Option<AuthLoginResponse>>(
        &resp.raw_response.unwrap_or_else(|| "{{}}".to_owned()),
      )?,
    })
  }
}
