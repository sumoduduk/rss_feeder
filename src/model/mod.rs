mod schedule_dc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Pool, Postgres, Row};
use std::collections::HashMap;

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct JobPost {
    pub title: String,
    pub link: String,
    pub detail: Value,
    pub category: String,
    #[serde(deserialize_with = "deserialize_rfc2822")]
    pub posted_on: DateTime<Utc>,
    pub posted_timestamp: i64,
}

fn deserialize_rfc2822<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc2822(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

pub enum RequestOperation {
    ReadAll,
    ReadByCategory(String),
    Insert(JobPost),
    Recent,
}

#[derive(Debug)]
pub enum ResponseOperation {
    Inserted(u64),
    RowsVec(Vec<JobPost>),
}

impl RequestOperation {
    async fn read_all(pool: &PgPool) -> Result<Vec<JobPost>, sqlx::Error> {
        let posts = sqlx::query_as(
            "SELECT title, link, detail, posted_on, posted_timestamp, category FROM job_posts",
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    async fn search_recent(pool: &PgPool) -> Result<Vec<JobPost>, sqlx::Error> {
        let posts = sqlx::query_as(
            "
            SELECT *
            FROM job_posts
            WHERE posted_on >= NOW() - INTERVAL '120 minutes'
            ",
        )
        .fetch_all(pool)
        .await;

        dbg!(&posts);
        Ok(posts?)
    }

    async fn read_by_category(pool: &PgPool, category: &str) -> Result<Vec<JobPost>, sqlx::Error> {
        let posts = sqlx::query_as::<Postgres, JobPost>(
            "
            SELECT title, link, detail, posted_on, posted_timestamp
            FROM job_posts
            WHERE category = $1 AND posted_on >= NOW() - INTERVAL '30 minutes'
            ",
        )
        .bind(category)
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    async fn insert(pool: &PgPool, post: &JobPost) -> Result<u64, sqlx::Error> {
        let inserted = sqlx::query(
            "INSERT INTO job_posts (title, link, detail, posted_on, posted_timestamp, category) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&post.title)
        .bind(&post.link)
        .bind(&post.detail)
        .bind(&post.posted_on)
        .bind(&post.posted_timestamp)
        .bind(&post.category)
        .execute(pool)
        .await?;
        dbg!(&inserted);

        let affected = inserted.rows_affected();

        Ok(affected)
    }

    pub async fn execute(&self, pool: &PgPool) -> Result<ResponseOperation, sqlx::Error> {
        match self {
            RequestOperation::ReadAll => {
                let posts = Self::read_all(pool).await?;
                Ok(ResponseOperation::RowsVec(posts))
            }
            RequestOperation::Recent => {
                let posts = Self::search_recent(pool).await?;
                Ok(ResponseOperation::RowsVec(posts))
            }
            RequestOperation::ReadByCategory(category) => {
                let posts = Self::read_by_category(pool, category).await?;
                Ok(ResponseOperation::RowsVec(posts))
            }
            RequestOperation::Insert(post) => {
                let inserted = Self::insert(pool, post).await?;
                Ok(ResponseOperation::Inserted(inserted))
            }
        }
    }
}
