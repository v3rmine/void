use crate::{
  auth::AuthEnv, computer::ComputerEnv, customer::CustomerEnv, dispatcher::DispatcherEnv,
  general::GeneralEnv, status::StatusEnv, Result,
};

use std::collections::HashMap;

#[cfg(feature = "logging")]
use crate::log::LogEnv;

pub type Response<T> = Result<InnerResponse<T>>;
#[derive(Debug, Clone)]
pub struct InnerResponse<T> {
  pub headers: HashMap<String, String>,
  pub status_code: u16,
  pub status: Status,
  pub raw_response: Option<T>,
}

pub trait ToResp {
  fn to_response(self) -> Response<String>;
}

impl ToResp for ureq::Response {
  fn to_response(self) -> Response<String> {
    let mut headers: HashMap<String, String> = HashMap::new();

    for k in self.headers_names().iter() {
      if let Some(v) = self.header(k) {
        headers.insert(k.to_owned(), v.to_owned());
      }
    }

    let status = match &self.status() {
      100..=199 => Status::Info,
      200..=299 => Status::Success,
      300..=399 => Status::Redirect,
      400..=499 => Status::ClientError,
      500..=599 => Status::ServerError,
      _ => Status::Unknown,
    };

    Ok(InnerResponse {
      headers,
      status_code: self.status(),
      status,
      raw_response: Some(self.into_string()?),
    })
  }
}

#[derive(Debug, Clone)]
pub enum Status {
  Info,        // 1xx
  Success,     // 2xx
  Redirect,    // 3xx
  ClientError, // 4xx
  ServerError, // 5xx
  Unknown,
}

#[derive(Debug, Clone)]
pub struct Shadow {
  pub email: String,
  pub password: String,
  pub session_uuid: String,
  pub shadow_uuid: String,
  pub inner: ShadowDynamic,
}

#[derive(Debug, Clone)]
pub struct ShadowDynamic {
  pub general: GeneralEnv,
  pub status: StatusEnv,
  #[cfg(feature = "logging")]
  pub log: LogEnv,
  pub computer: ComputerEnv,
  pub auth: AuthEnv,
  pub customer: CustomerEnv,
  pub dispatcher: DispatcherEnv,
}

impl Shadow {
  pub fn new(
    email: String,
    password: String,
    session_uuid: String,
    shadow_uuid: String,
  ) -> Result<Self> {
    Ok(Self {
      email,
      password,
      session_uuid,
      shadow_uuid,
      inner: ShadowDynamic::default()?,
    })
  }
  pub fn empty() -> Result<Self> {
    Ok(Self {
      email: String::new(),
      password: String::new(),
      session_uuid: String::new(),
      shadow_uuid: String::new(),
      inner: ShadowDynamic::default()?,
    })
  }
}

impl ShadowDynamic {
  pub fn default() -> Result<Self> {
    Ok(Self {
      general: GeneralEnv::default()?,
      status: StatusEnv::default()?,
      #[cfg(feature = "logging")]
      log: LogEnv::default(),
      computer: ComputerEnv::default(),
      auth: AuthEnv::default()?,
      customer: CustomerEnv::default()?,
      dispatcher: DispatcherEnv::default()?,
    })
  }
}
