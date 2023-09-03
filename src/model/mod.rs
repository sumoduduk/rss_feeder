use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug)]
struct JobPost {
    title: String,
    link: String,
    detail: HashMap<String, String>,
    posted_on: DateTime<Utc>,
    posted_timestamp: i64,
}

enum ResponseOperation {
    Inserted,
    AllRows,
    CategoryRows,
}

enum RequestOperation {
    ReadAll,
    ReadByCategory(String),
    Insert(JobPost),
}

impl RequestOperation {
    pub fn execute(&self, pool: &PgPool) -> Result<RequestOperation, sqlx::Error> {
        match self {
            RequestOperation::ReadAll => todo!(),
            RequestOperation::ReadByCategory(_) => todo!(),
            RequestOperation::Insert(_) => todo!(),
        }
    }
}
