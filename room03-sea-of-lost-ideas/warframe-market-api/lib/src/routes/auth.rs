use hyper::Body;
use serde::{Deserialize, Serialize};

use crate::{Error, schema::CurrentUser, WarframeMarket};

#[async_trait::async_trait]
pub trait Auth {
    async fn auth_login(&mut self, body: &LoginData) -> Result<CurrentUser, Error>;
    async fn auth_registration(&mut self, body: &RegistrationData) -> Result<CurrentUser, Error>;
    async fn auth_restore(&self, body: &RestoreData) -> Result<(), Error>;
}

#[async_trait::async_trait]
impl Auth for WarframeMarket {
    #[tracing::instrument]
    async fn auth_login(&mut self, body: &LoginData) -> Result<CurrentUser, Error> {
        let req = self.post("/auth/signin").body(Body::from(serde_json::to_string(body).map_err(Error::Desrialization)?))?;
        let resp = self.client.request(req).await?;
        serde_json::from_slice::<CurrentUser>(&hyper::body::to_bytes(resp.into_body()).await?).map_err(Error::Serialization)
    }

    #[tracing::instrument]
    async fn auth_registration(&mut self, body: &RegistrationData) -> Result<CurrentUser, Error> {
        let req = self.post("/auth/registration").body(Body::from(serde_json::to_string(body).map_err(Error::Desrialization)?))?;
        let resp = self.client.request(req).await?;
        serde_json::from_slice::<CurrentUser>(&hyper::body::to_bytes(resp.into_body()).await?).map_err(Error::Serialization)
    }

    #[tracing::instrument]
    async fn auth_restore(&self, body: &RestoreData) -> Result<(), Error> {
        let req = self.post("/auth/signin").body(Body::from(serde_json::to_string(body).map_err(Error::Desrialization)?))?;
        self.client.request(req).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginData {
    email: String,
    password: String,
    device_id: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistrationData {
    email: String,
    password: String,
    device_id: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestoreData {
    email: String,
    password: String,
    password_second: String,
    region: Option<String>,
    device_id: Option<String>,
    recaptcha: Option<String>
}