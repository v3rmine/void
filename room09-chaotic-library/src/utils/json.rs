use serde_json::Value;

/// Merges two JSON Values, with the second one taking precedence.
/// Fields in `b` will override fields in `a`.
pub fn merge_json_values(a: &Value, b: &Value) -> Value {
  match (a, b) {
    (Value::Object(a), Value::Object(b)) => {
      // If both are objects, merge recursively
      let mut result = a.clone();
      for (k, v) in b {
        if !a.contains_key(k) {
          // If key doesn't exist in a, just insert the value from b
          result.insert(k.clone(), v.clone());
        } else {
          // If key exists in both, recursively merge
          let a_value = a.get(k).unwrap();
          result.insert(k.clone(), merge_json_values(a_value, v));
        }
      }
      Value::Object(result)
    }
    (_, b) => {
      // For non-objects or when types don't match, b completely replaces a
      b.clone()
    }
  }
}

/// Parses frontmatter from a string.
pub fn parse_frontmatter<'i, T: serde::Deserialize<'i>>(
  input: &'i str,
) -> Result<T, serde_yml::Error> {
  serde_yml::from_str(input.splitn(3, "---").nth(1).unwrap_or_default())
}
