use hyper::Body;
use serde::{Deserialize, Serialize};

use crate::{Error, WarframeMarket, schema::{ItemShort, ItemFull}};

#[async_trait::async_trait]
pub trait Items {
    async fn items(&self) -> Result<ItemsList, Error>;
    async fn item(&self, url_name: &str) -> Result<ItemsItem, Error>;
    async fn item_orders(&self, url_name: &str, include_item: bool) -> Result<ItemsList, Error>;
    async fn item_dropsources(&self, url_name: &str, include_item: bool) -> Result<ItemsList, Error>;
}

#[async_trait::async_trait]
impl Items for WarframeMarket {
    #[tracing::instrument]
    async fn items(&self) -> Result<ItemsList, Error> {
        let req = self.get("/items").body(Body::empty())?;
        let resp = self.client.request(req).await?;
        serde_json::from_slice::<ItemsList>(&hyper::body::to_bytes(resp.into_body()).await?).map_err(Error::Serialization)
    }
    #[tracing::instrument]
    async fn item(&self, url_name: &str) -> Result<ItemsItem, Error> {
        let req = self.get(["/items/", url_name].concat()).body(Body::empty())?;
        let resp = self.client.request(req).await?;
        serde_json::from_slice::<ItemsItem>(&hyper::body::to_bytes(resp.into_body()).await?).map_err(Error::Serialization)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsList {
    pub payload: ItemListPayload
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemListPayload {
    pub items: Vec<ItemShort>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItem {
    pub payload: ItemsItemPayload
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItemPayload {
    pub item: ItemsItemWithId
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsItemWithId {
    id: String,
    items_in_set: Vec<ItemFull>
}