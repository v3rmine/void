use serde::Deserialize;
use url::Url;

use std::collections::HashMap;

use crate::constants::{DEFAULT_ACCEPT_ENCODING, DEFAULT_ACCEPT_LANGUAGE, DEFAULT_USER_AGENT};
use crate::{client::InnerResponse, Response, Shadow, ToResp};

#[derive(Debug, Clone)]
pub struct StatusEnv {
  pub url: Url,
}

default_url!(StatusEnv, crate::constants::DEFAULT_STATUS_ENDPOINT);

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponse {
  pub result: StatusResponseResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseResult {
  pub status_overall: StatusResponseOverall,
  pub status: Vec<StatusResponseByDatacenter>,
  pub incidents: Vec<StatusResponseIncident>,
  pub maintenance: StatusResponseMaintenance,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseOverall {
  pub updated: String,
  pub status: String,
  pub status_code: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseByDatacenter {
  pub id: String,
  pub name: String,
  pub updated: String,
  pub status: String,
  pub status_code: u16,
  pub containers: Vec<StatusResponseContainer>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseContainer {
  pub id: String,
  pub name: String,
  pub updated: String,
  pub status: String,
  pub status_code: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseMessage {
  pub details: String,
  pub state: u16,
  pub status: u16,
  pub datetime: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseIncident {
  pub name: String,
  pub _id: String,
  pub datetime_open: String,
  pub messages: Vec<StatusResponseMessage>,
  pub containers_affected: Vec<StatusResponseSummary>,
  pub components_affected: Vec<StatusResponseSummary>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseSummary {
  pub name: String,
  pub _id: String,
}

// TODO Find maintenance model
#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponseMaintenance {
  pub active: Vec<HashMap<String, String>>,
  pub upcoming: Vec<HashMap<String, String>>,
}

impl Shadow {
  pub fn status(&self) -> Response<StatusResponse> {
    let resp: InnerResponse<String> = ureq::get(&format!(
      "{}1.0/status/5bbcb1b0b0936904c004bbeb",
      &self.inner.status.url
    ))
    .set("User-Agent", DEFAULT_USER_AGENT)
    .set("Sec-Fetch-Site", "none")
    .set("Sec-Fetch-Mode", "no-cors")
    .set("Accept-Encoding", DEFAULT_ACCEPT_ENCODING)
    .set("Accept-Language", DEFAULT_ACCEPT_LANGUAGE)
    .call()
    .to_response()?;

    Ok(InnerResponse {
      headers: resp.headers,
      status: resp.status,
      status_code: resp.status_code,
      raw_response: serde_json::from_str::<Option<StatusResponse>>(
        &resp.raw_response.unwrap_or_else(|| "{{}}".to_owned()),
      )?,
    })
  }
}
