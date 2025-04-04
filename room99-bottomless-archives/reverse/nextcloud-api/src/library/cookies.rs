use headless_chrome::protocol::Method;
use headless_chrome::Tab;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: f64,
    pub http_only: bool,
    pub secure: bool,
    pub session: bool,
    pub same_site: Option<CookieSameSite>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CookieSameSite {
    Strict,
    Lax,
    Extended,
    None,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCookies {}
/*{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>
}*/

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturnObject {
    pub cookies: Vec<Cookie>,
}

impl Method for GetAllCookies {
    const NAME: &'static str = "Network.getAllCookies";
    type ReturnObject = GetCookiesReturnObject;
}

pub trait CookiesManagement {
    fn get_all_cookies(&self) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
impl CookiesManagement for Tab {
    fn get_all_cookies(&self) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
        Ok(self.call_method(GetAllCookies {})?.cookies)
    }
}

pub trait CookiesFns {
    fn get_cookie_by_name(&self, _name: &str) -> Option<Cookie> {
        unimplemented!()
    }
}
impl CookiesFns for Vec<Cookie> {
    fn get_cookie_by_name(&self, name: &str) -> Option<Cookie> {
        match self.iter().find(|x| x.name.eq(name)) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }
}
