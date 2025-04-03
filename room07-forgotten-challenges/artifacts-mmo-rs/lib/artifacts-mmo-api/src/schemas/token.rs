use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BearerToken(pub String);

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/generate_token_token__post>
#[derive(Debug, Clone, Deserialize)]
pub struct TokenSchema {
    pub token: BearerToken,
}
