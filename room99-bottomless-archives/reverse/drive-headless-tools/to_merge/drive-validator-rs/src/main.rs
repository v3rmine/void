use std::ffi::CString;

mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //https://accounts.google.com/o/oauth2/auth?access_type=offline&client_id=202264815644.apps.googleusercontent.com&redirect_uri=urn%3Aietf%3Awg%3Aoauth%3A2.0%3Aoob&response_type=code&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fdrive&state=1adcb06db170345cd24958627b3d81be
    let argv: Vec<String> = std::env::args().collect();
    let argv: Vec<&str> = argv.iter().map(|x| x.as_str()).collect();
    if argv.len() == 5 {
        let app_id = *if !argv.get(1).unwrap().eq(&"default") {
            argv.get(1)
        } else {
            Some(&"202264815644.apps.googleusercontent.com")
        }
        .expect("ERROR CODE -1");
        let scope = *if !argv.get(2).unwrap().eq(&"default") {
            argv.get(2)
        } else {
            Some(&"https://www.googleapis.com/auth/drive")
        }
        .expect("ERROR CODE -1");
        let mail = *argv.get(3).expect("ERROR CODE -1");
        let password = *argv.get(4).expect("ERROR CODE -1");
        println!(
            "{:?}",
            lib::DriveAuthorize(
                CString::new(app_id)?,
                CString::new(scope)?,
                CString::new(mail)?,
                CString::new(password)?,
                CString::new("")?,
            )
        );
    } else if argv.len() == 4 {
        let mail = *argv.get(1).expect("ERROR CODE -1");
        let password = *argv.get(2).expect("ERROR CODE -1");
        let url = *argv.get(3).expect("ERROR CODE -1");
        println!(
            "{:?}",
            lib::DriveAuthorize(
                CString::new("")?,
                CString::new("")?,
                CString::new(mail)?,
                CString::new(password)?,
                CString::new(url)?,
            )
        );
    } else {
        println!(
            "{:?}",
            lib::DriveAuthorize(
                CString::new("202264815644.apps.googleusercontent.com")?,
                CString::new("https://www.googleapis.com/auth/drive")?,
                CString::new("hello@hume.cloud")?,
                CString::new("Ahrp69GD")?,
                CString::new("")?,
            )
        );
    }

    Ok(())
}
