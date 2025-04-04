#![allow(dead_code)]
use headless_chrome::protocol::page::methods::{
    SetDownloadBehavior, SetDownloadBehaviorReturnObject,
};
use headless_chrome::protocol::{network, page};
use headless_chrome::Tab;

pub trait SetDownloadBehaviorTrait {
    fn page_set_download_behavior(
        &self,
        _behavior: &str,
        _download_path: Option<&str>,
    ) -> Result<SetDownloadBehaviorReturnObject, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
impl SetDownloadBehaviorTrait for Tab {
    fn page_set_download_behavior(
        &self,
        behavior: &str,
        download_path: Option<&str>,
    ) -> Result<SetDownloadBehaviorReturnObject, Box<dyn std::error::Error>> {
        Ok(self.call_method(SetDownloadBehavior {
            behavior,
            download_path,
        })?)
    }
}

pub trait PageEnable {
    fn page_enable(&self) -> Result<page::methods::EnableReturnObject, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
impl PageEnable for Tab {
    fn page_enable(&self) -> Result<page::methods::EnableReturnObject, Box<dyn std::error::Error>> {
        Ok(self.call_method(page::methods::Enable {})?)
    }
}

pub trait NetworkEnable {
    fn network_enable(
        &self,
    ) -> Result<network::methods::EnableReturnObject, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
impl NetworkEnable for Tab {
    fn network_enable(
        &self,
    ) -> Result<network::methods::EnableReturnObject, Box<dyn std::error::Error>> {
        Ok(self.call_method(network::methods::Enable {})?)
    }
}
