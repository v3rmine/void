use serde::Serialize;
use url::Url;

use crate::constants::{DEFAULT_ACCEPT_ENCODING, DEFAULT_ACCEPT_LANGUAGE, DEFAULT_USER_AGENT};
use crate::{Response, Result, Shadow, ToResp};

#[derive(Debug, Clone)]
pub struct LogEnv {
  url: Option<Url>,
  token: Option<String>,
}

default_url_w_token!(LogEnv);

#[derive(Serialize, Debug)]
pub struct LogQuery<'a, 'b, 'c> {
  pub name: &'a str,
  pub version: u32,
  pub timestamp: u64,
  pub privacy: &'a str,
  pub metadata: LogQueryMetadata<'b>,
  pub values: LogQueryValues<'c>,
}

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Debug)]
pub struct LogQueryMetadata<'a> {
  pub version: &'a str,
  pub packager_version: &'a str,
  pub launcher_version: &'a str,
  pub renderer_version: &'a str,
  pub user_environment: &'a str,
  pub os_family: &'a str,
  pub os_version: &'a str,
  pub session: &'a str,
  pub user_uuid: &'a str,
  pub main_display: &'a str,
  pub main_display_resolution: &'a str,
  pub cpu_model: &'a str,
  pub cpu_cores: &'a str,
  pub cpu_frequency: &'a str,
  pub total_memory: &'a str,
  pub graphic_card: &'a str,
}

#[derive(Serialize, Debug)]
pub struct LogQueryValues<'a> {
  pub status: &'a str,
}

impl Shadow {
  pub fn log(&self, query: LogQuery) -> Response<String> {
    let mut url = String::new();
    let mut token = String::new();

    if let Some(u) = &self.inner.log.url {
      url = u.to_string();
    } else {
      url = self.gen_log_url()?;
    }

    if let Some(t) = &self.inner.log.token {
      token = t.to_owned();
    } else {
      token = self.gen_log_token()?;
    }

    let resp = ureq::post(&format!("{}/log", &url))
      .auth_kind("Bearer", &token)
      .set("Content-Type", "application/json")
      .set("User-Agent", DEFAULT_USER_AGENT)
      .set("X-Shadow-Uuid", &self.shadow_uuid)
      .set("Sec-Fetch-Site", "none")
      .set("Sec-Fetch-Mode", "no-cors")
      .set("Accept-Encoding", DEFAULT_ACCEPT_ENCODING)
      .set("Accept-Language", DEFAULT_ACCEPT_LANGUAGE)
      .send_json(json!(query));

    resp.to_response()
  }

  pub fn gen_log_url(&self) -> Result<String> {
    // TODO gen_log
    unimplemented!()
  }

  pub fn gen_log_token(&self) -> Result<String> {
    // TODO gen_log
    unimplemented!()
  }
}
