use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Pool, Postgres, Row};

use crate::{utils::datetime_to_string, xml_parse::JobPost};

pub async fn get_all(pool: Data<Pool<Postgres>>) -> impl Responder {
    let mut conn = pool.get_ref().acquire().await.unwrap();

    let rows = sqlx::query(
        r#"
        select * from job_posts
        "#,
    )
    .fetch_all(&mut conn)
    .await
    .unwrap();

    let mut job_posts = Vec::new();

    for row in rows {
        let detail_json: Value = row.try_get("detail").unwrap();
        let detail = serde_json::from_value(detail_json).unwrap();

        let time: DateTime<Utc> = row.try_get("posted_on").unwrap();

        let time_str = datetime_to_string(Some(time)).unwrap();

        let job_post = JobPost {
            title: row.try_get("title").unwrap(),
            link: row.try_get("link").unwrap(),
            detail,
            posted_on: time_str,
            posted_timestamp: row.try_get("posted_timestamp").unwrap(),
        };

        job_posts.push(job_post);
    }

    HttpResponse::Ok().json(job_posts)
}
