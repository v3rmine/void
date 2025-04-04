use crate::library::devtoolproto::{NetworkEnable, PageEnable};
use headless_chrome::Tab;

pub trait HideMe {
    fn undetect(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl HideMe for Tab {
    fn undetect(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.page_enable()?;
        self.network_enable()?;
        Ok(())
    }
}
