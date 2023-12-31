#![allow(unused_imports)]
mod model;
mod router;
mod schdule;
mod utils;
mod xml_parse;

use xml_parse::parse_xml;

use actix_web::{
    web::{get, Data},
    App, HttpResponse, HttpServer, Responder,
};

use actix_cors::Cors;
use reqwest::Client;
use rss::Channel;
use scraper::{Html, Selector};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use router::get_all;

use crate::router::{
    begin_scrape, insert_db, job_search::get_job, read_by_catergory, recent_search, start_task,
};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
    uri: String,
}

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let uri = env::var("URI").expect("URI not found");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    let app_state = AppState { pool, uri };

    let addr = "0.0.0.0:8080";

    println!("listening at port : {}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .route("/", get().to(index))
            .route("/get_all", get().to(get_all))
            .route("/recent", get().to(recent_search))
            .route("/begin_process", get().to(begin_scrape))
            .route("/start_task", get().to(start_task))
            .route("/search_job", get().to(get_job))
            .service(read_by_catergory)
            .service(insert_db)
            .app_data(Data::new(app_state.clone()))
            .wrap(cors)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>Hello world!</h1>")
}
