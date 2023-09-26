use actix_web::{web, App, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use tokio::task;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct JobPost {
    title: String,
    link: String,
    detail: HashMap<String, String>,
    posted_on: String,
    posted_timestamp: i64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = Pool::<Postgres>::connect("postgres://user:pass@localhost/db").await?;

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/create", web::post().to(create))
            .route("/sync", web::get().to(sync))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello world!")
}

async fn create(pool: web::Data<Pool<Postgres>>, item: web::Json<JobPost>) -> impl Responder {
    let _ = sqlx::query!(
        r#"
    INSERT INTO job_posts (title, link, detail, posted_on, posted_timestamp)
    VALUES ($1, $2, $3, $4, $5)
    "#,
        item.title,
        item.link,
        &item.detail,
        item.posted_on,
        item.posted_timestamp
    )
    .execute(pool.get_ref())
    .await;

    HttpResponse::Ok().finish()
}

// async fn sync(pool: web::Data<Pool<Postgres>>) -> impl Responder {
//     let client = Client::new();
//
//     let res = client.get("https://example.com/jobs").send().await?;
//     let jobs: Vec<JobPost> = res.json().await?;
//
//     for job in jobs {
//         let pool = pool.clone();
//
//         task::spawn(async move {
//             let _ = sqlx::query!(
//                 r#"
//         INSERT INTO job_posts (title, link, detail, posted_on, posted_timestamp)
//         VALUES ($1, $2, $3, $4, $5)
//         "#,
//                 job.title,
//                 job.link,
//                 &job.detail,
//                 job.posted_on,
//                 job.posted_timestamp
//             )
//             .execute(&pool)
//             .await;
//         });
//     }
//
//     HttpResponse::Ok().finish()
// }

fn anot() {
    // let file = Client::new().get(uri).send().await?.bytes().await?;
    //
    // let bye = &file[..];

    // let channel = Channel::read_from(&response[..]).unwrap();

    let path = Path::new("./example/rss.xml");

    let file = File::open(path)?;

    let reader = BufReader::new(&file);
    // let channel = Channel::read_from(BufReader::new(&file))?;

    let item = parse_xml(reader)?;
    let len = item.len();
    println!("lengtt of the vec : {}", len);

    let target_path = Path::new("./example/job_post_after.json");

    let json_str = serde_json::to_string_pretty(&item).unwrap();
    fs::write(target_path, json_str).expect("Unable to write file");
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
