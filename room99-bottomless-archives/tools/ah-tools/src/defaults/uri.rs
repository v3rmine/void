use std::collections::HashMap;

pub fn collect_params(uri: &str) -> HashMap<&str, &str> {
    uri.split('&')
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| {
            let e = e.split('=').collect::<Vec<&str>>();
            (*e.get(0).unwrap(), *e.get(1).unwrap())
            //(e.get(0).unwrap().clone(), e.get(1).unwrap().clone())
        })
        .collect::<HashMap<&str, &str>>()
}