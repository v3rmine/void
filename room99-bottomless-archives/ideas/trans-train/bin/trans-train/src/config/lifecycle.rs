use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lifecycle {
    #[serde(default = "Vec::new")]
    pub requires: Vec<String>,
    #[serde(flatten)]
    pub extra_params: Option<LifecycleExtra>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum LifecycleExtra {
    Input { params: Vec<InputParam> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputParam {
    #[serde(rename = "as")]
    pub alias: Option<String>,
    #[serde(default = "Vec::new")]
    pub depends_on: Vec<String>,
    #[serde(flatten)]
    pub extra: InputParamExtra,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputParamExtra {
    Glob { glob: String },
    Path { path: String },
}
