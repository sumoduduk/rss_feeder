#![allow(unused_imports)]
mod model;
mod router;
mod utils;
mod xml_parse;

use xml_parse::parse_xml;

use actix_web::{
    web::{get, Data},
    App, HttpResponse, HttpServer, Responder,
};
use dotenvy::dotenv;
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

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    let addr = "127.0.0.1:8080";

    println!("listening at port : {}", addr);

    HttpServer::new(move || {
        App::new()
            .route("/", get().to(index))
            .route("/get_all", get().to(get_all))
            .app_data(Data::new(pool.clone()))
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
