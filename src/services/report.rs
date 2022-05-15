use std::sync::{Arc, Mutex};

use elasticsearch::{Elasticsearch, SearchParts};
use handlebars::Handlebars;
use serde_json::json;

use crate::{es_client, PlebisError, Query, RESULTS_TEMPLATE};
use crate::types::es_envelope::EsEnvelope;
use crate::types::report::{Report, ReportHighlight};
use crate::types::results_payload::{ResultEnvelope, ResultItem};

pub async fn report(
    erowid_id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let reg = Handlebars::new();

    let es = es_client();

    let search_response =
        es
            .search(SearchParts::None)
            .body(json!({
                "query": {
                    "match": {
                        "meta.erowidId": &erowid_id,
                    }
                },
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
        search_response.json::<EsEnvelope<Report, ReportHighlight>>()
            .await
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
            title: format!("{} - Plebis", &erowid_id),
            query: erowid_id,
            results: result_items.clone(),
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

    dbg!(&tpl_val);

    reg
        .render_template(
            RESULTS_TEMPLATE,
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
