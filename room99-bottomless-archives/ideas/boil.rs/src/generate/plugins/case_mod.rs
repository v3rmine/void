use std::collections::HashMap;

use tera::{Result, Value};

use super::TeraFilter;

pub fn all<'a>() -> Vec<(&'static str, TeraFilter<'a>)> {
    let mut result = Vec::new();
    result.push(("TitleCase", &case::title_case as TeraFilter<'a>));
    result.push(("togglecase", &case::toggle_case as TeraFilter<'a>));
    result.push(("flatcase", &case::flat_case as TeraFilter<'a>));
    result.push(("alternatingcase", &case::alternating_case as TeraFilter<'a>));
    result.push(("snake_case", &case::snake_case as TeraFilter<'a>));
    result.push((
        "screaming_snake_case",
        &case::screaming_snake_case as TeraFilter<'a>,
    ));
    result.push(("kebab-case", &case::kebab_case as TeraFilter<'a>));
    result.push(("COBOL-CASE", &case::cobol_case as TeraFilter<'a>));
    result.push(("Train-Case", &case::train_case as TeraFilter<'a>));
    result.push(("PascalCase", &case::pascal_case as TeraFilter<'a>));
    result.push(("camelCase", &case::camel_case as TeraFilter<'a>));
    result
}

pub mod case {
    use convert_case::{Case, Casing};

    use super::{HashMap, Result, Value};

    pub fn title_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("TitleCase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Title)))
    }
    pub fn toggle_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("togglecase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Toggle)))
    }
    pub fn flat_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("flatcase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Flat)))
    }
    pub fn alternating_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("alternatingcase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Alternating)))
    }
    pub fn snake_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("snake_case", "value", String, value);
        Ok(Value::String(s.to_case(Case::Snake)))
    }
    pub fn screaming_snake_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("screaming_snake_case", "value", String, value);
        Ok(Value::String(s.to_case(Case::ScreamingSnake)))
    }
    pub fn kebab_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("kebab-case", "value", String, value);
        Ok(Value::String(s.to_case(Case::Kebab)))
    }
    pub fn cobol_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("COBOL-CASE", "value", String, value);
        Ok(Value::String(s.to_case(Case::Cobol)))
    }
    pub fn train_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("Train-Case", "value", String, value);
        Ok(Value::String(s.to_case(Case::Train)))
    }
    pub fn pascal_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("PascalCase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Pascal)))
    }
    pub fn camel_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("camelCase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Camel)))
    }
}
