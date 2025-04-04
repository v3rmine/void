mod error;

pub use error::Error;
pub(crate) mod routes;
pub mod schema;

pub mod prelude {
    pub use crate::WarframeMarket;

    pub use crate::routes::Auth;
    pub use crate::routes::Items;
}

use hyper::{client::HttpConnector, http::request, Client, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

#[derive(Debug)]
pub struct WarframeMarket {
    base_url: &'static str,
    client: Client<HttpsConnector<HttpConnector>>,
    pub language: &'static str,
    pub platform: &'static str,
}

impl WarframeMarket {
    fn request(&self, url: impl AsRef<str>) -> request::Builder {
        Request::builder()
            .uri([self.base_url, url.as_ref()].concat())
            .header("Language", self.language)
            .header("Platform", self.platform)
    }
    fn get(&self, url: impl AsRef<str>) -> request::Builder {
        self.request(url).method("GET")
    }
    fn post(&self, url: impl AsRef<str>) -> request::Builder {
        self.request(url).method("POST")
    }
}

impl Default for WarframeMarket {
    fn default() -> Self {
        Self {
            client: Client::builder().build(
                HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_only()
                    .enable_http1()
                    .build(),
            ),
            base_url: "https://api.warframe.market/v1",
            language: "en",
            platform: "pc"
        }
    }
}
