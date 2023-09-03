use std::io::BufReader;

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use crate::{
    model::{JobPost, RequestOperation, ResponseOperation},
    utils::datetime_to_string,
    xml_parse::parse_xml,
};

type DataPool = Data<Pool<Postgres>>;

pub async fn get_all(pool: DataPool) -> impl Responder {
    let conn = pool.get_ref();

    let response_db = RequestOperation::ReadAll.execute(conn).await;

    match response_db {
        Ok(ResponseOperation::RowsVec(data)) => HttpResponse::Ok().json(data),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/get_category/{category}")]
pub async fn read_by_catergory(pool: DataPool, category: Path<String>) -> impl Responder {
    let conn = pool.get_ref();

    let response_db = RequestOperation::ReadByCategory(category.into_inner())
        .execute(conn)
        .await;

    match response_db {
        Ok(ResponseOperation::RowsVec(data)) => HttpResponse::Ok().json(data),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/insert")]
pub async fn insert_db(pool: DataPool, body: Json<JobPost>) -> impl Responder {
    println!("traced");
    let pool = pool.get_ref();
    let body = body.into_inner();
    dbg!(&body);

    let row_affected = RequestOperation::Insert(body).execute(pool).await;

    let value = json!({
    "message": "OK"
    });

    match row_affected {
        Ok(ResponseOperation::Inserted(_)) => HttpResponse::Ok().json(value),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

// #[post("/insert")]
// pub async fn insert_db(body: Json<crate::xml_parse::JobPost>) -> impl Responder {
//     dbg!(body);
//     HttpResponse::Ok().finish()
// }

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
