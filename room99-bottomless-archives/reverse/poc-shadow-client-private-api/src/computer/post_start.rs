use crate::constants::{
  DEFAULT_ACCEPT_ENCODING, DEFAULT_ACCEPT_LANGUAGE, DEFAULT_SHADOW_AGENT, DEFAULT_USER_AGENT,
};
use crate::{Response, Shadow, ToResp};

impl Shadow {
  pub fn start_vm(&mut self) -> Response<String> {
    // TODO self.inner.computer.url
    ureq::post(&format!(
      "{}/shadow/vm/start",
      "https://prod.gap.pa1.blade-group.fr:2443/"
    ))
    .set("Connection", "keep-alive")
    .set("User-Agent", DEFAULT_USER_AGENT)
    .set("X-Shadow-Agent", DEFAULT_SHADOW_AGENT)
    .set("X-Shadow-Uuid", &self.shadow_uuid)
    .set("Sec-Fetch-Site", "none")
    .set("Sec-Fetch-Mode", "no-cors")
    .set("Accept-Encoding", DEFAULT_ACCEPT_ENCODING)
    .set("Accept-Language", DEFAULT_ACCEPT_LANGUAGE)
    .set(
      "Authorization",
      &format!(
        "Token {}",
        &(match &self.inner.computer.token {
          Some(x) => x.to_owned(),
          None => {
            self.inner.computer.token = Some(self.auth_login()?.raw_response.unwrap().token);
            self.inner.computer.token.as_ref().unwrap().to_owned()
          }
        })
      ),
    )
    .call()
    .to_response()
  }
}
