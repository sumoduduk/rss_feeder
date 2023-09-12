use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    model::schedule_dc::{ScheduleOperation, ScheduleTask},
    utils::reqwst_to_server,
};

pub async fn operate_task(
    str_time: &str,
    expected_time: DateTime<Utc>,
    pool: &Pool<Postgres>,
) -> HttpResponse {
    //check if task are exist
    let read_exist = ScheduleOperation::GetSchedule {
        task_time: str_time.to_string(),
    }
    .execute(pool)
    .await;

    match read_exist {
        Ok(exist_result) => match exist_result {
            true => reqwst_to_server().await,
            false => {
                let task_time = ScheduleTask {
                    expected_time,
                    time_str: str_time.to_string(),
                    done: true,
                };

                let is_inserted = ScheduleOperation::AddSchedule(task_time)
                    .execute(pool)
                    .await;
                match is_inserted {
                    Ok(inserted) => match inserted {
                        true => {
                            let msg = json!({"messeage" : "success inserting task"});
                            HttpResponse::Ok().json(msg)
                        }

                        false => HttpResponse::InternalServerError().finish(),
                    },

                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
