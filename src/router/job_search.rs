use actix_web::{
    web::{Bytes, Data, Query},
    HttpResponse, Responder,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
    xml_parse::{parse_xml, JobPost},
    AppState,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobList {
    title: String,
    link: String,
    posted_timestamp: u32,
    budget: Option<String>,
    hourly: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryMap {
    recency: Option<String>,
    q: Option<String>,
}

pub async fn get_job(
    query_data: Query<HashMap<String, String>>,
    state: Data<AppState>,
) -> impl Responder {
    let uri = &state.get_ref().uri;
    let inner_query = query_data.into_inner();

    dbg!(&inner_query);
    let bytes_data = request_bytes(uri, inner_query).await;

    match bytes_data {
        Ok(data) => match populate_data(data) {
            Ok(arr_data) => HttpResponse::Ok().json(arr_data),
            _ => HttpResponse::InternalServerError().finish(),
        },
        _ => HttpResponse::InternalServerError().finish(),
    }
}

async fn request_bytes(
    uri: &str,
    query_data: HashMap<String, String>,
) -> Result<Bytes, reqwest::Error> {
    let response_byte = Client::new()
        .get(uri)
        .query(&query_data)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(response_byte)
}

fn populate_data(byte_data: Bytes) -> eyre::Result<Vec<Value>> {
    let result_data = parse_xml(&byte_data[..])?;

    let list_job: Vec<Value> = result_data
        .into_iter()
        .map(|j| {
            let budget = j.detail.get("Budget");
            let hourly = j.detail.get("Hourly Range");

            let title_job: Vec<_> = j.title.split("- Upwo").collect();

            let mut price = "Unknown".to_string();

            match (budget, hourly) {
                (Some(b), None) => {
                    price = format!("Budget : {}", b);
                }
                (None, Some(h)) => {
                    price = format!("Hourly Range : {}", h);
                }
                (_, _) => (),
            }
            let response_json = json!({ "title": title_job[0], "link": j.link, "price": price });
            response_json
        })
        .collect();

    Ok(list_job)
}
