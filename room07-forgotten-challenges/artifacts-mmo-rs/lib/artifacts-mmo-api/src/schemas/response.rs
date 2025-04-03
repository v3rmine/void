use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseSchema<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponseSchema<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub size: u32,
    pub pages: u32,
}
