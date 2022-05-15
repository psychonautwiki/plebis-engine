#![feature(async_closure, type_alias_impl_trait)]

use std::convert::Infallible;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use elasticsearch::{
    Elasticsearch, Error,
    http::transport::Transport,
};
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::services::search;

mod services;
mod types;

pub const RESULTS_TEMPLATE: &'static str = include_str!("../templates/results.hbs");

#[derive(Debug, Serialize, Deserialize)]
pub enum PlebisError {
    Db(String),
    DataError(String),
    DataConversionError(String),
    RenderError(String),
}

impl warp::reject::Reject for PlebisError {}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub q: String,
}

fn es_client() -> Elasticsearch {
    Elasticsearch::new(
        Transport::single_node(
            "http://127.0.0.1:9200",
        ).unwrap()
    )
}

fn with_es_client(
    es: Arc<Mutex<Elasticsearch>>,
) -> impl Filter<Extract=(Arc<Mutex<Elasticsearch>>, ), Error=Infallible> + Clone {
    warp::any().map(move || es.clone())
}

#[tokio::main]
async fn main() {
    let es =
        Arc::new(
            Mutex::new(
                es_client(),
            ),
        );

    let search =
        warp::path!("search")
            .and(warp::query::<Query>())
            .and(with_es_client(es.clone()))
            .and_then(search::search);

    let get_routes =
        warp::fs::dir("static")
            .or(
                warp::path!("search" / "assets")
                    .and(warp::fs::dir("static/assets"))
            )
            .or(
                warp::path!("search" / "font")
                    .and(warp::fs::dir("static/font"))
            )
            .or(search);

    let routes =
        warp::get()
            .and(
                get_routes,
            );

    println!("i cannot see and it is cold");
    println!("if you can read this help");
    println!("its dark and I am lost");
    println!("▪");
    println!("listening on port 7171");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 7171))
        .await;
}
