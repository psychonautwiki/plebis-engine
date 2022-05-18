use serde::{Serialize, Deserialize};

pub type ResultItems = Vec<ResultItem>;

#[derive(Default, Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub struct ResultItemTag {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub struct ResultItem {
    pub id: String,
    pub title: String,
    pub display_text: String,
    pub link: String,
    pub tags: Vec<ResultItemTag>,
    pub entry_tags: Vec<ResultItemTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub struct ResultEnvelope<T, P> {
    pub total_results: i64,
    pub title: String,
    pub query: String,
    pub data: T,
    pub extra: P,
}
