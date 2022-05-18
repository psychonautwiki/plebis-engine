use std::sync::{Arc, Mutex};

use elasticsearch::{Elasticsearch, SearchParts};
use handlebars::Handlebars;
use serde_json::json;

use crate::{es_client, PlebisError, Query, RESULTS_TEMPLATE};
use crate::types::es_envelope::EsEnvelope;
use crate::types::report::{Report, ReportHighlight, ReportProcessed};
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

    if hits.len() != 1 {
        return Err(warp::reject::custom(
            PlebisError::DataError(
                String::from("Received more than one result"),
            ),
        ));
    }

    let mut report = hits[0].clone();

    report.source.processed = {
        let processed_body =
            report
                .source
                .body
                .split("\r\n\r\n")
                // remove leading and trailing whitespaces
                .map(|section| section.trim_start().trim_end())
                // wrap in <p> tags
                .map(|section| format!("<p>{}</p>", section))
                // replace \r\n with <br>
                .map(|section| section.replace("\r\n", "<br>"))
                .collect::<Vec<_>>();

        Some(
            ReportProcessed {
                body: processed_body.join("\n"),
            }
        )
    };

    let result_env =
        ResultEnvelope {
            total_results: search_result.hits.total.value,
            title: format!("{} - Plebis", &erowid_id),
            query: erowid_id,
            data: report.source.clone(),
            extra: ResultItem::from(&report),
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

    reg
        .render_template(
            //REPORT_TEMPLATE,
            &std::fs::read_to_string("templates/report.hbs").unwrap(),
            &dbg!(tpl_val),
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
