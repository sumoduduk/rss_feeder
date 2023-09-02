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

async fn sync(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let client = Client::new();

    let res = client.get("https://example.com/jobs").send().await?;
    let jobs: Vec<JobPost> = res.json().await?;

    for job in jobs {
        let pool = pool.clone();

        task::spawn(async move {
            let _ = sqlx::query!(
                r#"
        INSERT INTO job_posts (title, link, detail, posted_on, posted_timestamp)
        VALUES ($1, $2, $3, $4, $5)
        "#,
                job.title,
                job.link,
                &job.detail,
                job.posted_on,
                job.posted_timestamp
            )
            .execute(&pool)
            .await;
        });
    }

    HttpResponse::Ok().finish()
}
