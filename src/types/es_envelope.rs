use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsEnvelope<T, H> {
    pub took: i64,
    #[serde(rename = "timed_out")]
    pub timed_out: bool,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub hits: Hits<T, H>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub total: i64,
    pub successful: i64,
    pub skipped: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hits<T, H> {
    pub total: Total,
    #[serde(rename = "max_score")]
    pub max_score: f64,
    pub hits: Vec<Hit<T, H>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Total {
    pub value: i64,
    pub relation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit<T, H> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_score")]
    pub score: f64,
    #[serde(rename = "_ignored")]
    pub ignored: Vec<String>,
    #[serde(rename = "_source")]
    pub source: T,
    pub highlight: H,
}
