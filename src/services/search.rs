use std::sync::{Arc, Mutex};

use elasticsearch::{Elasticsearch, SearchParts};
use handlebars::Handlebars;
use serde_json::json;

use crate::{es_client, PlebisError, Query, RESULTS_TEMPLATE};
use crate::types::es_envelope::EsEnvelope;
use crate::types::report::{Report, ReportHighlight};
use crate::types::results_payload::{ResultEnvelope, ResultItem};

pub async fn search(
    query: Query,
    //es: Arc<Mutex<Elasticsearch>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let es = es_client();

    //let es =
    //    match es.lock() {
    //        Ok(es) => es,
    //        Err(e) => {
    //            return Err(
    //                warp::reject::custom(
    //                    PlebisError::Db(
    //                        "Failed to obtain db instance",
    //                    ),
    //                ),
    //            );
    //        }
    //    };

    let search_response =
        es
            .search(SearchParts::None)
            .body(json!({
                "query": {
                    "multi_match": {
                        "query": &query.q,
                        "fields": ["title", "body^7"]
                    }
                },
                "highlight": {
                    "pre_tags": ["<b>"],
                    "post_tags": ["</b>"],
                    "fields": {
                        "title": {
                            "number_of_fragments": 1,
                            "fragment_size": 100
                        },
                        "body": {
                            "fragment_size": 100,
                            "number_of_fragments": 3,
                            "order": ""
                        }
                    }
                }
            }))
            .allow_no_indices(true)
            .send()
            .await
            .map_err(|err|
                warp::reject::custom(
                    PlebisError::Db(
                        format!("{:?}", err),
                    ),
                )
            )?;

    let search_result =
        dbg!(search_response.json::<EsEnvelope<Report, ReportHighlight>>()
            .await)
            .map_err(|err|
                         warp::reject::custom(
                             PlebisError::DataError(
                                 format!("{:?}", err),
                             ),
                         ),
            )?;

    let hits = search_result.hits.hits;

    let result_items: Vec<ResultItem> =
        hits
            .iter()
            .map(|item| item.into())
            .collect();

    let result_env =
        ResultEnvelope {
            total_results: search_result.hits.total.value,
            title: format!("{} - Plebis", &query.q),
            query: query.q.clone(),
            data: result_items.clone(),
            extra: Option::<()>::None,
        };

    let tpl_val =
        serde_json::to_value(&result_env)
            .map_err(|err|
                         warp::reject::custom(
                             PlebisError::DataConversionError(
                                 format!("{:?}", err),
                             ),
                         ),
            )?;

    let reg = Handlebars::new();

    reg
        .render_template(
            //RESULTS_TEMPLATE,
            &std::fs::read_to_string("templates/results.hbs").unwrap(),
            &tpl_val,
        )
        .map(|body|
                 warp::reply::html(
                     body,
                 ),
        )
        .map_err(|err|
                     warp::reject::custom(
                         PlebisError::RenderError(
                             format!("{:?}", err),
                         ),
                     ),
        )
}
