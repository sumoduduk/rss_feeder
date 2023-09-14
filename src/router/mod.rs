pub mod job_search;

use std::{collections::HashMap, io::BufReader};

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use chrono::{Timelike, Utc};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use crate::{
    model::{JobPost, RequestOperation, ResponseOperation},
    schdule::{
        check_schedule, formated_time, return_task_hour, schedule_operation::operate_task,
        to_central_time,
    },
    utils::{datetime_to_string, reqwst_to_server},
    xml_parse::process_request,
    AppState,
};

pub async fn start_task(state: Data<AppState>) -> impl Responder {
    let pool = &state.get_ref().pool;

    let now = Utc::now();
    let time_central = to_central_time(&now);

    match time_central {
        Some(time_some) => {
            let hour = time_some.hour();
            let at_interval = check_schedule(hour);
            if !at_interval {
                reqwst_to_server().await
            } else {
                let hour_now = time_some.hour();
                let returned_time = return_task_hour(hour_now);
                let formated = formated_time(&time_some, returned_time);

                operate_task(&formated, time_some.into(), pool).await
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all(pool: Data<AppState>) -> impl Responder {
    let conn = &pool.get_ref().pool;

    let response_db = RequestOperation::ReadAll.execute(conn).await;

    match response_db {
        Ok(ResponseOperation::RowsVec(data)) => HttpResponse::Ok().json(data),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn recent_search(pool: Data<AppState>) -> impl Responder {
    let conn = &pool.get_ref().pool;

    let response_db = RequestOperation::Recent.execute(conn).await;

    match response_db {
        Ok(ResponseOperation::RowsVec(data)) => HttpResponse::Ok().json(data),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/get_category/{category}")]
pub async fn read_by_catergory(pool: Data<AppState>, category: Path<String>) -> impl Responder {
    let conn = &pool.get_ref().pool;

    let response_db = RequestOperation::ReadByCategory(category.into_inner())
        .execute(conn)
        .await;

    match response_db {
        Ok(ResponseOperation::RowsVec(data)) => HttpResponse::Ok().json(data),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/insert")]
pub async fn insert_db(pool: Data<AppState>, body: Json<JobPost>) -> impl Responder {
    let conn = &pool.get_ref().pool;
    let body = body.into_inner();

    let row_affected = RequestOperation::Insert(body).execute(conn).await;

    let value = json!({
    "message": "OK"
    });

    match row_affected {
        Ok(ResponseOperation::Inserted(_)) => HttpResponse::Ok().json(value),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn begin_scrape(state: Data<AppState>) -> impl Responder {
    let pool = &state.get_ref().pool;
    let uri = &state.get_ref().uri;

    let mut params = HashMap::with_capacity(1);
    params.insert("sort", "recency");

    let bytes_response = Client::new().get(uri).query(&params).send().await;

    match bytes_response {
        Ok(resp) => {
            let bytes_res = resp.bytes().await;

            match bytes_res {
                Ok(is_bytes) => {
                    let result = process_request(&is_bytes[..], pool).await;

                    match result {
                        Ok(success_msg) => {
                            let res = json!({
                            "message": success_msg,
                            "status": "OK"
                            });

                            HttpResponse::Ok().json(res)
                        }
                        Err(_) => HttpResponse::InternalServerError().json(json!({
                        "message": "error spawn worker task"
                        })),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().json(json!({
                "message": "error processing bytes"
                })),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({
        "message": "error from URI"
        })),
    }
}
