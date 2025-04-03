use std::collections::HashMap;

use urlencoding::encode;

pub fn query_from_hash_map<T: Into<HashMap<String, String>>>(hashmap: T) -> String {
    let params: HashMap<String, String> = hashmap.into();

    if params.is_empty() {
        String::new()
    } else {
        [
            "?{}",
            &params
                .iter()
                .map(|(k, v)| [&encode(k), "=", &encode(v)].concat())
                .collect::<Vec<String>>()
                .join("&"),
        ]
        .concat()
    }
}

pub fn bool_as_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}
