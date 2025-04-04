use super::{sleep, urlencode, var, HideMe, Params};
use crate::library::devtoolproto::SetDownloadBehaviorTrait;
use headless_chrome::{
    browser::default_executable, browser::tab::element::Element, browser::tab::Tab, Browser,
    LaunchOptionsBuilder,
};
use std::ffi::OsStr;
use std::io::Read;
use std::sync::Arc;

macro_rules! ostr_vec {
    [$($e:expr),*] => {
        vec![$(OsStr::new($e)),*]
    };
}

fn sa_authorize(
    username: &str,
    password: &str,
    project_name: &str,
    sa_id: &str,
    timeout_short: u64,
    headless: bool,
) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let download_dir = format!("{}\\Downloads\\", var("USERPROFILE").unwrap());
    #[cfg(not(target_os = "windows"))]
    let download_dir = format!("{}/Downloads/", var("HOME").unwrap());
    let options = LaunchOptionsBuilder::default()
        .path(Some(default_executable().unwrap()))
        .headless(headless)
        .sandbox(true)
        .args(ostr_vec![
            "--disable-gpu",
            ////"--disable-background-networking", // Disable the downloads
            "--disable-infobars",
            "--ignore-certifcate-errors",
            "--ignore-certifcate-errors-spki-list",
            "--enable-features=NetworkService,NetworkServiceInProcess",
            "--disable-background-timer-throttling",
            "--disable-backgrounding-occluded-windows",
            "--disable-breakpad",
            "--disable-client-side-phishing-detection",
            "--disable-default-apps",
            "--disable-dev-shm-usage",
            "--disable-extensions",
            "--disable-features=site-per-process,TranslateUI,BlinkGenPropertyTrees",
            "--disable-hang-monitor",
            "--disable-ipc-flooding-protection",
            "--disable-popup-blocking",
            "--disable-prompt-on-repost",
            "--disable-renderer-backgrounding",
            "--disable-sync",
            "--force-color-profile=srgb",
            "--metrics-recording-only",
            "--no-first-run",
            "--enable-automation",
            "--password-store=basic",
            "--use-mock-keychain",
            "--safebrowsing-disable-download-protection",
            ////"--disable-web-security",
            "--trusted-download-sources=console.cloud.google.com",
            "--reduce-security-for-testing",
            "--enable-devtools-experiments",
            "--disable-notifications",
            "--allow-running-insecure-content",
            "'--user-agent='Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/65.0.3312.0 Safari/537.36'",
            "--disable-accelerated-2d-canvas=true"
        ])
        .build()
        .unwrap();
    let browser = Browser::new(options)?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;
    debug!("Timeout of {}ms", var("HEADLESS_TIMEOUT").unwrap());
    let timeout =
        std::time::Duration::from_millis(var("HEADLESS_TIMEOUT").unwrap().parse::<u64>()?);
    let redirect_url = format!("https://console.cloud.google.com/iam-admin/serviceaccounts/details/{sa_id}?project={project_name}&supportedpurview=project&pli=1",
                               sa_id = sa_id,
                               project_name = project_name);
    let url = format!("https://accounts.google.com/ServiceLogin/identifier?service=cloudconsole&continue={redir}&followup={redir}&flowName=GlifWebSignIn&flowEntry=AddSession",
                      redir = urlencode(redirect_url.clone()));

    tab.undetect()?;
    if headless {
        debug!("Download directory: {}", download_dir);
        tab.page_set_download_behavior("allow", Some(download_dir.as_str()))?;
    }

    // Login
    info!("LOGGING IN");
    debug!("Navigate: {}", url.as_str());
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;
    //println!("{:?}", tab.find_element("body > *:first-of-type")?.call_js_fn("() => document.querySelectorAll('input')", false));
    if headless {
        tab.wait_for_element_with_custom_timeout("input#Email", timeout)?
    } else {
        tab.wait_for_element_with_custom_timeout("input#identifierId", timeout)?
    }
    .click()?;

    tab.type_str(username)?;
    debug!("Action: Typing username");
    sleep(timeout_short);
    if headless {
        tab.wait_for_element_with_custom_timeout("input#next", timeout)?
    } else {
        tab.wait_for_element_with_custom_timeout("#identifierNext", timeout)?
    }
    .click()?;
    debug!("Action: Exiting username");
    sleep(timeout_short * 3);
    //println!("{:?}", tab.find_element("body > *:first-of-type")?.call_js_fn("() => document.querySelectorAll('input')", false));
    if headless {
        tab.wait_for_element_with_custom_timeout("input#Passwd", timeout)?
    } else {
        tab.wait_for_element_with_custom_timeout("#password input", timeout)?
    }
    .click()?;
    tab.type_str(password)?;
    sleep(timeout_short * 2);
    debug!("Action: Typing password");
    if headless {
        tab.wait_for_element_with_custom_timeout("input#signIn", timeout)?
    } else {
        tab.wait_for_element_with_custom_timeout("#passwordNext", timeout)?
    }
    .click()?;

    debug!("Action: Exiting password");
    // Using SA
    tab.wait_until_navigated()?;
    debug!("Navigate: {}", redirect_url);
    tab.wait_for_element_with_custom_timeout("#default-action-bar button:first-of-type", timeout)?
        .click()?;
    info!("Action: Generate certificate JSON");
    sleep(timeout_short);
    tab.wait_for_element_with_custom_timeout(
        "cfc-disable[cfciamcheck=\"iam.serviceAccountKeys.create\"] > div > button",
        timeout,
    )?
    .click()?;
    sleep(timeout_short);
    tab.wait_for_element_with_custom_timeout("mat-radio-button:nth-of-type(1)", timeout)?
        .click()?;
    sleep(timeout_short);
    tab.wait_for_element_with_custom_timeout(
        "cfc-create-service-account-key-dialog cfc-progress-button button",
        timeout,
    )?
    .click()?;
    sleep(timeout_short * 3);
    tab.wait_for_element_with_custom_timeout(
        "mat-dialog-container .mat-dialog-actions button",
        timeout,
    )?
    .click()?;
    // SA File
    let json: Element = tab.wait_for_element_with_custom_timeout(
        "cfc-service-account-key-list section input",
        timeout,
    )?;
    let json = json
        .call_js_fn(
            "() => document.querySelector('cfc-service-account-key-list section input').value",
            false,
        )?
        .value
        .ok_or("QUERY ERROR")?
        .as_str()
        .ok_or("QUERY ERROR")?
        .to_owned();

    info!("Action: Generate certificate P12");
    sleep(timeout_short * 2);
    tab.wait_for_element_with_custom_timeout(
        "cfc-disable[cfciamcheck=\"iam.serviceAccountKeys.create\"] > div > button",
        timeout,
    )?
    .click()?;
    sleep(timeout_short);
    tab.wait_for_element_with_custom_timeout("mat-radio-button:nth-of-type(2)", timeout)?
        .click()?;
    sleep(timeout_short);
    tab.wait_for_element_with_custom_timeout(
        "cfc-create-service-account-key-dialog cfc-progress-button button",
        timeout,
    )?
    .click()?;
    sleep(timeout_short * 3);
    tab.wait_for_element_with_custom_timeout(
        "mat-dialog-container .mat-dialog-actions button",
        timeout,
    )?
    .click()?;
    // SA File
    let p12: Element = tab.wait_for_element_with_custom_timeout(
        "cfc-service-account-key-list section input",
        timeout,
    )?;
    let p12 = p12
        .call_js_fn(
            "() => document.querySelector('cfc-service-account-key-list section input').value",
            false,
        )?
        .value
        .ok_or("QUERY ERROR")?
        .as_str()
        .ok_or("QUERY ERROR")?
        .to_owned();
    info!("json: {}", json.clone());
    info!("p12: {}", p12.clone());
    sleep(timeout_short * 3);
    let fname_json = format!("{}-{}.json", project_name, &json[..12]);
    let fname_p12 = format!("{}-{}.p12", project_name, &p12[..12]);
    //let text: Element = tab.wait_for_element_with_custom_timeout("mat-dialog-container .mat-form-field-infix input", timeout)?;
    let path_json = format!("{}{}", download_dir.clone(), fname_json);
    let path_p12 = format!("{}{}", download_dir, fname_p12);

    info!("{}", path_json);
    info!("{}", path_p12);
    let path_json = std::path::Path::new(path_json.as_str());
    let path_p12 = std::path::Path::new(path_p12.as_str());
    let json = std::fs::read_to_string(path_json)?;
    let mut file = std::fs::File::open(path_p12)?;
    let mut buffer = Vec::<u8>::new();
    info!("Size read in p12 file: {}", file.read_to_end(&mut buffer)?);
    std::fs::remove_file(path_json)?;
    std::fs::remove_file(path_p12)?;
    Ok((json, buffer))
}

pub fn sa_setup(msg: Params) -> Option<(String, Vec<u8>)> {
    let timeout = msg.timeout.unwrap_or(500);
    match sa_authorize(
        msg.username,
        msg.password,
        msg.project_name,
        msg.sa_id,
        timeout,
        msg.headless,
    ) {
        Ok(x) => Some(x),
        Err(e) => {
            warn!("SA authorize: {}", e);
            None
        }
    }
}
