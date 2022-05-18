use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::types::es_envelope::Hit;
use crate::types::results_payload::{ResultItem, ResultItemTag};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportProcessed {
    pub body: String,
}

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
    pub processed: Option<ReportProcessed>,
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
            match hit.highlight.as_ref().map(|hl| hl.title.as_ref()).flatten() {
                Some(title) if title.len() > 0 =>
                    title.join(""),
                _ => hit.source.title,
            };

        let display_text =
            match hit.highlight.map(|hl| hl.body).flatten() {
                Some(body) if body.len() > 0 =>
                    [
                        body.as_slice(),
                        &[String::from("")],
                    ].concat().join(" … "),
                _ => format!("{} …", &hit.source.body[0..300]),
            };

        let link =
            format!(
                "/report/{}",
                hit.source.meta.erowid_id,
            );

        let mut substance_set = HashSet::new();

        for substance_info in hit.source.substance_info.iter() {
            let substance_meta =
                vec!(
                    &substance_info.form,
                    &substance_info.method,
                    &substance_info.amount,
                )
                    .iter()
                    .filter_map(|item|
                        if *item == "" {
                            None
                        } else {
                            Some((*item).clone())
                        }
                    )
                    .collect::<Vec<_>>();

            substance_set.insert(
                format!(
                    "{}{}",
                    substance_info.substance,
                    if substance_meta.len() > 0 {
                        format!(" [{}]", substance_meta.join(", "))
                    } else {
                        "".to_string()
                    }
                )
            );
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

        let mut entry_tags = Vec::<ResultItemTag>::new();

        if let Some(gender) = hit.source.meta.gender {
            entry_tags
                .push(
                    ResultItemTag {
                        label: gender,
                    },
                );
        }

        if let Some(age) = hit.source.meta.age {
            entry_tags
                .push(
                    ResultItemTag {
                        label: format!("{}y", age),
                    },
                );
        }

        if let Some(year) = hit.source.meta.year {
            entry_tags
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
            id: hit.source.meta.erowid_id.to_string(),
            title,
            display_text,
            link,
            tags: substance_tags,
            entry_tags,
        }
    }
}

impl From<&Hit<Report, ReportHighlight>> for ResultItem {
    fn from(hit: &Hit<Report, ReportHighlight>) -> Self {
        hit.clone().into()
    }
}
