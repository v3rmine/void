use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangInItem {
    pub item_name: String,
    pub description: String,
    pub wiki_link: Option<String>,
    pub drop: Vec<LangInItemDrop>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangInItemDrop {
    pub name: String,
    pub link: Option<String>
}
