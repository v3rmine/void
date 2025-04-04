#[cfg(feature = "security")]
#[macro_export]
macro_rules! response {
    ($code:ident, $json:expr) => {
        || -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let key = ah_tools::reexports::rsa::Rsa::public_key_from_pem(crate::routes::EXTERNAL)?;
            let response = serde_json::to_string(&$json).unwrap();
            let response = crate::security::public_encrypt(&key, response.as_bytes())?;
            let response = crate::reexports::encode(&serde_json::to_string(&response).unwrap());
            let response = HttpResponse::$code()
                .content_type("application/json")
                .body(&response);
            Ok(response)
        }()
        .map_err(|_| ())?
    };
}
#[cfg(feature = "security")]
#[macro_export]
macro_rules! response_unsecure {
    ($code:ident, $json:expr) => {
        || -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let response = serde_json::to_string(&$json).unwrap();
            let response = HttpResponse::$code()
                .content_type("application/json")
                .body(&response);
            Ok(response)
        }()
        .map_err(|_| ())?
    };
}
#[cfg(not(feature = "security"))]
#[macro_export]
macro_rules! response {
    ($code:ident, $json:expr) => {
        || -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let response = serde_json::to_string(&$json).unwrap();
            let response = HttpResponse::$code()
                .content_type("application/json")
                .body(&response);
            Ok(response)
        }()
        .map_err(|_| ())?
    };
}

