use serde::{Serialize, Deserialize};

pub type ResultItems = Vec<ResultItem>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultItemTag {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    pub title: String,
    pub display_text: String,
    pub link: String,
    pub tags: Vec<ResultItemTag>,
    #[serde(rename = "obstrusiveTags")]
    pub obstrusive_tags: Vec<ResultItemTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultEnvelope<T> {
    pub title: String,
    pub query: String,
    pub results: T,
}
