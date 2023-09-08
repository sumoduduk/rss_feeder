use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, FromRow)]
pub struct ScheduleTask {
    expected_time: DateTime<Utc>,
    done: bool,
}

pub enum ScheduleOperation {
    AddSchedule(ScheduleTask),
    GetSchedule { task_time: DateTime<Utc> },
}

impl ScheduleOperation {
    async fn insert_task(
        &self,
        task: &ScheduleTask,
        pool: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        let inserted = sqlx::query(
            "INSERT INTO schedule_task (created_at, expected_time, done)
                        VALUES ($1, $2, $3)",
        )
        .bind(task.expected_time)
        .bind(task.done)
        .execute(pool)
        .await?;

        let inserted = inserted.rows_affected();
        Ok(inserted)
    }

    async fn get_schedule(
        task_time: &DateTime<Utc>,
        pool: &Pool<Postgres>,
    ) -> Result<i64, sqlx::Error> {
        let db_id: ScheduleTask =
            sqlx::query_as("SELECT id FROM schedule_task WHERE expected_time = $1")
                .bind(task_time)
                .fetch_one(pool)
                .await?;

        dbg!(&db_id);

        todo!();
    }
}
