use std::io::BufReader;

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use crate::{
    model::{JobPost, RequestOperation, ResponseOperation},
    schdule::check_schedule,
    utils::{datetime_to_string, reqwst_to_server},
    xml_parse::{parse_xml, process_request},
    AppState,
};

pub async fn get_all(pool: Data<AppState>) -> impl Responder {
    let conn = &pool.get_ref().pool;

    let response_db = RequestOperation::ReadAll.execute(conn).await;

    match response_db {
        Ok(ResponseOperation::RowsVec(data)) => HttpResponse::Ok().json(data),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn recent_search(pool: Data<AppState>) -> impl Responder {
    println!("test");
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
    let bytes_response = Client::new().get(uri).send().await;

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

pub async fn start_task(state: Data<AppState>) -> impl Responder {
    let pool = &state.get_ref().pool;

    let at_interval = check_schedule();
    if !at_interval {
        reqwst_to_server().await;
    } else {
    }
}

#[get("/print_file")]
pub async fn print_file() -> impl Responder {
    // let file = Client::new().get(uri).send().await?.bytes().await?;
    //
    // let bye = &file[..];

    // let channel = Channel::read_from(&response[..]).unwrap();

    let path = std::path::Path::new("./example/rss.xml");

    let file = std::fs::File::open(path).unwrap();

    let reader = BufReader::new(&file);
    // let channel = Channel::read_from(BufReader::new(&file))?;

    let item = parse_xml(reader).unwrap();

    let len = item.len();
    println!("lengtt of the vec : {}", len);

    let target_path = std::path::Path::new("./example/job_post_after.json");

    let json_str = serde_json::to_string_pretty(&item).unwrap();
    std::fs::write(target_path, json_str).expect("Unable to write file");

    HttpResponse::Ok().finish()
}
