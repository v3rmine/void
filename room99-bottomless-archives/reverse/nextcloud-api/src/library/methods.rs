use super::cookies::{CookiesFns, CookiesManagement};
use super::var;
use super::{private_decrypt, Tomb};
use super::{QueryCreateUser, QueryGetToken};
use crate::PRIVATE;
use headless_chrome::{
    browser::default_executable, browser::tab::Tab, Browser, LaunchOptionsBuilder,
};
use std::sync::Arc;

pub fn get_tokens(
    query: QueryGetToken,
) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    // Browser setup
    let username = query.username.to_owned();
    let password = query.password.to_owned();
    let mut options = LaunchOptionsBuilder::default();
    let options = options
        .path(Some(default_executable().unwrap()))
        .sandbox(true);
    #[cfg(target_os = "windows")]
    let options = options.headless(false);
    #[cfg(not(target_os = "windows"))]
    let options = options.headless(true);
    let options = options.build().unwrap();

    let browser = Browser::new(options)?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;
    let timeout =
        std::time::Duration::from_millis(var("HEADLESS_TIMEOUT").unwrap().parse::<u64>()?);
    let url = format!("{}/login", var("BASE_NC_URL")?);

    // Begin
    debug!("Navigate: {}", url.as_str());
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;
    tab.wait_for_element_with_custom_timeout("input#user", timeout)?
        .click()?;
    tab.type_str(username.as_str())?;
    tab.wait_for_element("input#password")?.click()?;
    tab.type_str(password.as_str())?;
    tab.wait_for_element("input#submit")?.click()?;
    tab.wait_until_navigated()?;
    let cookies = tab.get_all_cookies()?;
    let session = cookies.get_cookie_by_name("nc_session_id");
    let token = cookies.get_cookie_by_name("nc_token");
    let username = cookies.get_cookie_by_name("nc_username");
    info!("{:?}", cookies);
    if session.is_none() || token.is_none() || username.is_none() {
        bail!("ERROR NO COOKIES")
    } else {
        Ok((
            session.unwrap().value,
            token.unwrap().value,
            username.unwrap().value,
        ))
    }
}

pub fn create_user(query: QueryCreateUser) -> Result<(), Box<dyn std::error::Error>> {
    // Browser setup
    let self_private = openssl::rsa::Rsa::private_key_from_pem(PRIVATE)?;
    let username = var("NC_ADMIN_USERNAME").unwrap();
    let username = String::from_utf8(base64::decode(
        &private_decrypt(
            &self_private,
            serde_json::from_str::<Tomb>(String::from_utf8(base64::decode(&username)?)?.as_str())?,
        )?
        .value,
    )?)?;
    let password = var("NC_ADMIN_PASSWORD").unwrap();
    let password = String::from_utf8(base64::decode(
        &private_decrypt(
            &self_private,
            serde_json::from_str::<Tomb>(String::from_utf8(base64::decode(&password)?)?.as_str())?,
        )?
        .value,
    )?)?;
    let mut options = LaunchOptionsBuilder::default();
    let options = options
        .path(Some(default_executable().unwrap()))
        .sandbox(true);
    #[cfg(target_os = "windows")]
    let options = options.headless(false);
    #[cfg(not(target_os = "windows"))]
    let options = options.headless(true);
    let options = options.build().unwrap();

    let browser = Browser::new(options)?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;
    let timeout =
        std::time::Duration::from_millis(var("HEADLESS_TIMEOUT").unwrap().parse::<u64>()?);
    let url = format!("{}/login", var("BASE_NC_URL")?);

    // Begin
    debug!("Navigate: {}", url.as_str());
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;
    tab.wait_for_element_with_custom_timeout("input#user", timeout)?
        .click()?;
    tab.type_str(username.as_str())?;
    tab.wait_for_element("input#password")?.click()?;
    tab.type_str(password.as_str())?;
    tab.wait_for_element("input#submit")?.click()?;
    tab.wait_until_navigated()?;
    tab.navigate_to("https://files.hume.cloud/settings/users")?;
    tab.wait_until_navigated()?;
    tab.wait_for_element("#new-user-button")?.click()?;
    tab.wait_for_element("#newusername")?.click()?;
    tab.type_str(query.username)?;
    tab.wait_for_element("#newuserpassword")?.click()?;
    tab.type_str(query.password)?;
    tab.wait_for_element("#newsubmit")?.click()?;
    Ok(())
}
