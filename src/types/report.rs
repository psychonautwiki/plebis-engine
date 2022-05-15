use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::types::es_envelope::Hit;
use crate::types::results_payload::{ResultItem, ResultItemTag};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportHighlight {
    pub body: Option<Vec<String>>,
    pub title: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub meta: Meta,
    pub author: String,
    pub body: String,
    pub erowid_notes: Vec<String>,
    pub pull_quotes: Vec<String>,
    pub substance: String,
    pub substance_info: Vec<SubstanceInfo>,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub year: Option<i64>,
    pub erowid_id: i64,
    pub gender: Option<String>,
    pub age: Option<i64>,
    pub published: String,
    pub views: Option<i64>,
    pub erowid_attributes: Option<ErowidAttributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErowidAttributes {
    pub categories: Vec<Category>,
    pub attributes: Vec<Attribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub name: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceInfo {
    pub amount: String,
    pub method: String,
    pub substance: String,
    pub form: String,
}

impl From<Hit<Report, ReportHighlight>> for ResultItem {
    fn from(hit: Hit<Report, ReportHighlight>) -> Self {
        let title =
            match hit.highlight.title {
                Some(title) if title.len() > 0 =>
                    title.join(""),
                _ => hit.source.title,
            };

        let display_text =
            match hit.highlight.body {
                Some(body) if body.len() > 0 =>
                    [
                        body.as_slice(),
                        &[String::from("")],
                    ].concat().join(" … "),
                _ => format!("{} …", &hit.source.body[0..300]),
            };

        let link =
            format!(
                "http://erowid.org.global.prod.fastly.net/experiences/exp.php?ID={}",
                hit.source.meta.erowid_id,
            );

        let mut substance_set = HashSet::new();

        for substance_info in hit.source.substance_info.iter() {
            substance_set.insert(substance_info.substance.clone());
        }

        let mut substance_tags =
            substance_set
                .iter()
                .map(|substance|
                    ResultItemTag {
                        label: substance.clone(),
                    }
                )
                .collect::<Vec<ResultItemTag>>();

        let mut obstrusive_tags = Vec::<ResultItemTag>::new();

        obstrusive_tags
            .push(
                ResultItemTag {
                    label: format!("{}", hit.source.meta.erowid_id),
                },
            );

        if let Some(gender) = hit.source.meta.gender {
            obstrusive_tags
                .push(
                    ResultItemTag {
                        label: gender,
                    },
                );
        }

        if let Some(age) = hit.source.meta.age {
            obstrusive_tags
                .push(
                    ResultItemTag {
                        label: format!("{}y", age),
                    },
                );
        }

        if let Some(year) = hit.source.meta.year {
            obstrusive_tags
                .push(
                    ResultItemTag {
                        label: format!("{}", year),
                    },
                );
        }

        substance_tags.sort_by(|a, b|
            a.label.cmp(&b.label)
        );

        ResultItem {
            title,
            display_text,
            link,
            tags: substance_tags,
            obstrusive_tags,
        }
    }
}

impl From<&Hit<Report, ReportHighlight>> for ResultItem {
    fn from(hit: &Hit<Report, ReportHighlight>) -> Self {
        hit.clone().into()
    }
}
