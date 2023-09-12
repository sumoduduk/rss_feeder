use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, FromRow)]
pub struct ScheduleTask {
    pub expected_time: DateTime<Utc>,
    pub time_str: String,
    pub done: bool,
}

#[derive(Debug, FromRow)]
struct TaskRow {
    id: i64,
    created_at: DateTime<Utc>,
    expected_time: Option<DateTime<Utc>>,
    done: Option<bool>,
    time_str: Option<String>,
}

pub enum ScheduleOperation {
    AddSchedule(ScheduleTask),
    GetSchedule { task_time: String },
}

impl ScheduleOperation {
    async fn insert_task(task: &ScheduleTask, pool: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
        let inserted = sqlx::query(
            "INSERT INTO schedule_task (expected_time, done, time_str)
                        VALUES ($1, $2, $3)",
        )
        .bind(task.expected_time)
        .bind(task.done)
        .bind(&task.time_str)
        .execute(pool)
        .await?;

        let inserted = inserted.rows_affected();
        Ok(inserted)
    }

    async fn _get_all(pool: &Pool<Postgres>) -> Result<Vec<ScheduleTask>, sqlx::Error> {
        let all: Vec<ScheduleTask> = sqlx::query_as(
            "
                SELECT expected_time, time_str, done FROM schedule_task
            ",
        )
        .fetch_all(pool)
        .await?;
        dbg!(&all);
        Ok(all)
    }

    async fn get_schedule(task_time: &str, pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
        let db_id: TaskRow = sqlx::query_as(
            "
                SELECT * FROM schedule_task 
                WHERE time_str = $1
            ",
        )
        .bind(task_time)
        .fetch_one(pool)
        .await?;

        dbg!(&db_id);

        Ok(db_id.id)
    }

    pub async fn execute(&self, pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
        match self {
            ScheduleOperation::AddSchedule(task_struct) => {
                let inserted_row = Self::insert_task(task_struct, pool).await;

                match inserted_row {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            ScheduleOperation::GetSchedule { task_time } => {
                let row_fetched = Self::get_schedule(task_time, pool).await;
                match row_fetched {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{parse_datetime_timezone, string_to_datetime};
    use chrono::{Datelike, FixedOffset, Timelike, Utc};
    use dotenvy::dotenv;
    use sqlx::{Pool, Postgres};
    use std::{env, str::FromStr};

    // #[tokio::test]
    // async fn test_insert_task() {
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //     let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();
    //
    //     let time_now = Utc::now();
    //     let str_time = format!(
    //         "{}_{}_{}",
    //         time_now.month(),
    //         time_now.day(),
    //         time_now.hour()
    //     );
    //
    //     let task = ScheduleTask {
    //         expected_time: time_now,
    //         time_str: str_time,
    //         done: false,
    //     };
    //
    //     let inserted = ScheduleOperation::insert_task(&task, &pool).await;
    //     assert!(inserted.is_ok());
    // }

    #[tokio::test]
    async fn test_get_all() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

        let getall = ScheduleOperation::_get_all(&pool).await;

        assert!(getall.is_ok())
    }

    #[tokio::test]
    async fn test_onel() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

        let str_time = "2023-03-03T12:00:00Z";

        let date: DateTime<Utc> = DateTime::parse_from_rfc3339(str_time).unwrap().into();
        let string_time = format!("{}_{}_{}", date.month(), date.day(), date.hour());

        let result = ScheduleOperation::get_schedule(&string_time, &pool)
            .await
            .unwrap();

        assert_eq!(26, result)
    }
}
