//!#[crate_type = "cdylib"]
//!#[crate_id = "r"]
extern crate headless_chrome;

use std::ffi::CString;
use std::sync::Arc;
use std::time::Duration;

use headless_chrome::protocol::dom::Node;
use headless_chrome::{
    browser::default_executable,
    //protocol::page::ScreenshotFormat,
    browser::tab::element::Element,
    browser::tab::Tab,
    Browser,
    LaunchOptionsBuilder,
};

//use std::io::prelude::*;

macro_rules! debug {
    ($x:expr, $y:expr) => {
        #[cfg(debug_assertions)]
        println!("{}: \u{001b}[31m{}\u{001b}[0m", $x, $y);
    };
}

fn drive_authorize(
    app_id: &str,
    scope: &str,
    username: &str,
    password: &str,
    _url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let options = LaunchOptionsBuilder::default()
        .path(Some(default_executable().unwrap()))
        .headless(false)
        .build()
        .unwrap();
    let browser = Browser::new(options)?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;

    let base_url = "https://accounts.google.com/o/oauth2/auth?";
    let access_type = "access_type=offline";
    let client_id = app_id;
    let redirect_uri = "redirect_uri=urn%3Aietf%3Awg%3Aoauth%3A2.0%3Aoob";
    let response_type = "response_type=code";
    //let mut scopes: String = scope.iter().map(|x| format!("{} ", x)).collect();
    let mut scopes = scope.to_owned();
    //copes.pop();
    scopes = scopes
        .replace(":", "%3A")
        .replace("/", "%2F")
        .replace(" ", "%20");
    let scopes = format!("{}{}", "scope=", scopes);
    let mut url = format!(
        "{}{}&client_id={}&{}&{}&{}",
        base_url, access_type, client_id, redirect_uri, response_type, scopes
    );
    if _url != "" {
        url = _url.to_string();
    }
    debug!("url", url.as_str());
    tab.navigate_to(url.as_str())?;
    // Username
    tab.wait_for_element("input#identifierId")?.click()?;
    tab.type_str(username)?;
    /*
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("1uname.png"))?).write_all(screen.as_mut_slice())?;
    */
    debug!("status", "Typing username");
    debug!("value", username);
    tab.find_element("#identifierNext")?.click()?;
    debug!("status", "Exiting username");
    /*
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("2ident.png"))?).write_all(screen.as_mut_slice())?;
    */
    // Password
    std::thread::sleep(Duration::from_secs(2));
    tab.wait_for_element("[name=\"password\"][type=\"password\"]")
        .expect("Password input not found")
        .click()?;
    tab.type_str(password)?;
    debug!("status", "Typing password");
    debug!("value", password);
    /*
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("3pass.png"))?).write_all(screen.as_mut_slice())?;
    */
    tab.find_element("#passwordNext")?.click()?;
    debug!("status", "Exiting password");
    /*
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("4auth.png"))?).write_all(screen.as_mut_slice())?;
    */
    // Validation
    /*
    tab.wait_for_element("[data-custom-id=\"oauthScopeDialog-allow\"]")?.click()?;
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("submit.png"))?).write_all(screen.as_mut_slice())?;
    */
    //std::thread::sleep(Duration::from_secs(1));
    tab.wait_for_element("#submit_approve_access[role=\"button\"]")?
        .click()?;
    /*
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None,true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("5code.png"))?).write_all(screen.as_mut_slice())?;
    */
    debug!("status", "Approval 2");
    // Token
    //std::thread::sleep(Duration::from_secs(1));
    /*
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new("6token.png"))?).write_all(screen.as_mut_slice())?;
    */
    debug!("search", "textarea");
    let x: Element = tab.wait_for_element("textarea[readonly]")?;
    debug!("status", "Searching textarea");
    let mut text: Vec<Node> = x
        .get_description()
        .expect("Error textarea is empty")
        .children
        .expect("No children found");
    text.retain(|x| x.node_name == "#text");
    Ok(text[0].clone().node_value)
}

#[allow(non_snake_case)]
#[link(name = "ext", kind = "static")]
pub extern "C" fn DriveAuthorize(
    app_id: CString,
    scope: CString,
    username: CString,
    password: CString,
    url: CString,
) -> CString {
    match drive_authorize(
        app_id.to_str().unwrap_or(""),
        scope.to_str().unwrap_or(""),
        username.to_str().unwrap_or(""),
        password.to_str().unwrap_or(""),
        url.to_str().unwrap_or(""),
    ) {
        Ok(x) => CString::new(x).unwrap_or(CString::new("").unwrap()),
        Err(_) => CString::new("").unwrap(),
    }
}
