use crate::macros::{export_scoped_macro, pub_use_mod};

pub_use_mod!(version);
pub_use_mod!(list_container);

export_scoped_macro!(simple_get {
    ($docker_conn:ident, $req_path:expr, $return_type:ty) => {{
        let response = $docker_conn
            .client
            .get($docker_conn.format_uri($req_path)?)
            .await?;
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let return_type = serde_json::from_slice::<$return_type>(&body)?;
        return_type
    }};
});

// export_scoped_macro!(simple_req {
//     ($docker_conn:ident, $req:expr, $return_type:ty) => {{
//         let response = $docker_conn
//             .client
//             .request($req)
//             .await?;
//         let body = hyper::body::to_bytes(response.into_body()).await?;
//         let return_type = serde_json::from_slice::<$return_type>(&body)?;
//         return_type
//     }};
// });
