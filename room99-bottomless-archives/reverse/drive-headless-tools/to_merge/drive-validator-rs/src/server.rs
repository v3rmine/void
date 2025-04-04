extern crate actix_web;
extern crate actix_web_codegen;

mod lib;

use actix_web::{
    App, HttpServer, web, Responder
};
use actix_web_codegen::get;
use std::ffi::CString;

#[get("/{app_id}/{scope}/{user}/{pass}")]
fn index(info: web::Path<(String,String,String,String)>) -> impl Responder {
    let app_id = urldecode(info.0.as_str());
    let scope = urldecode(info.1.as_str());
    let user = urldecode(info.2.as_str());
    let pass = urldecode(info.3.as_str());
    let code = lib::DriveAuthorize(
        CString::new(info.0.clone()).unwrap(),
        CString::new(info.1.clone()).unwrap(),
        CString::new(info.2.clone()).unwrap(),
        CString::new(info.3.clone()).unwrap(),
        CString::new("").unwrap()
    );

    format!("{}", code.to_str().unwrap())
}

fn urlencode<'a>(url: &str) -> String {
    let mut scopes = url.to_owned();
    //copes.pop();
    scopes = scopes
        .replace(":", "%3A")
        .replace("/", "%2F")
        .replace(" ", "%20");

    return scopes;
}

fn urldecode<'a>(url: &str) -> String {
    let mut scopes = url.to_owned();
    //copes.pop();
    scopes = scopes
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%20", " ");

    return scopes;
}

fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(
        index //web::resource("/{username}/{pass}").to(index)
    ));
    println!("{}", urlencode("202264815644.apps.googleusercontent.com"));
    println!("{}", urlencode("https://www.googleapis.com/auth/drive"));
    println!("{}", urlencode("hume.cloud@hume.cloud"));
    println!("{}", urlencode("Ahrp69GD"));
    server.bind("127.0.0.1:6660")?.run()
}