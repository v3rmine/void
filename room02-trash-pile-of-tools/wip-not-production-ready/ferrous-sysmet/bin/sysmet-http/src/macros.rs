#[macro_export]
macro_rules! static_files_server {
    ($name:ident, $dir:ident, $hashes:ident, $content_type:expr) => {
        pub async fn $name(
            ::axum::extract::Path(path): ::axum::extract::Path<String>,
        ) -> impl ::axum::response::IntoResponse {
            use axum::{
                body::{self, Empty, Full},
                http::{header, HeaderValue, Response, StatusCode},
            };

            let path = path.trim_start_matches('/');
            let not_found = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap();

            if !$hashes.contains_key(path) {
                return not_found;
            }

            match $dir.get_file(
                &$hashes
                    .get(path)
                    .map(|(real_path, _hash)| real_path)
                    .unwrap(),
            ) {
                None => not_found,
                Some(file) => Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_str($content_type).unwrap(),
                    )
                    .body(body::boxed(Full::from(file.contents())))
                    .unwrap(),
            }
        }
    };
}

#[macro_export]
macro_rules! generate_hashes {
    ($name:ident, $dir:ident) => {
        ::once_cell::sync::Lazy::new(|| {
            use base64::encode;
            use sha2::{Digest, Sha256};

            $dir.files()
                .map(|file| {
                    let path = file.path().to_path_buf();
                    let hash = encode(Sha256::digest(file.contents()));
                    let mut asset_path = path.clone();
                    asset_path.set_extension(
                        [
                            // NOTE: Fix for / in base64 encoded shasum
                            &hash[..8].replace('/', "_"),
                            ".",
                            &asset_path.extension().unwrap().to_string_lossy(),
                        ]
                        .concat(),
                    );

                    (
                        asset_path.to_string_lossy().to_string(),
                        (path, ["sha256-", &hash].concat()),
                    )
                })
                .collect()
        })
    };
}
