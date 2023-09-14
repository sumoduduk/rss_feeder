use actix_web::{
    web::{Bytes, Data, Query},
    HttpResponse, Responder,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    xml_parse::{parse_xml, JobPost},
    AppState,
};

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

fn populate_data(byte_data: Bytes) -> eyre::Result<Vec<JobPost>> {
    let result_data = parse_xml(&byte_data[..])?;
    Ok(result_data)
}
