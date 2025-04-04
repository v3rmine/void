use serde::Deserialize;

pub fn serde_is_valid_and_contain<'a, T>(json: &'a str, key: &str, val: &str) -> bool
    where
        T: Deserialize<'a>,
{
    let key = key.to_owned();
    let val = val.to_owned();
    let result = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json).unwrap();
    serde_json::from_str::<'a, T>(json).is_ok()
        && result.contains_key(key.as_str())
        && result
        .get(key.as_str())
        .unwrap()
        .eq(&serde_json::Value::String(val))
}

#[macro_export]
macro_rules! ss {
    ($type:ty, $val:expr) => {
        serde_json::from_str::<$type>($val).unwrap()
    };
}