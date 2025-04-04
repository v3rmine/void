use std::ops::Deref;

pub fn urlencode<T: Deref<Target = str>>(url: T) -> String {
    let scopes = url.to_owned();
    scopes
        .replace(":", "%3A")
        .replace("/", "%2F")
        .replace(" ", "%20")
        .replace("?", "%3F")
        .replace("&", "%26")
        .replace("=", "%3D")
}

pub fn urldecode<T: Deref<Target = str>>(url: T) -> String {
    let scopes = url.to_owned();
    scopes
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%20", " ")
        .replace("%3F", "?")
        .replace("%26", "&")
        .replace("%3D", "=")
}