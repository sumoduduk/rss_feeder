use actix_web::rt;
use rss::Item;
use sqlx::{Pool, Postgres};

use crate::{
    model::{JobPost as JobPostDB, RequestOperation},
    utils::{parse_date, string_to_datetime},
};

use super::{mapped_detail::mapped_detail, JobPost};

pub async fn worker_task(items: Vec<Item>, pool: &Pool<Postgres>) {
    for item in items {
        let _ = rt::spawn(async move {
            let desc = item.description;

            match desc {
                Some(description) => {
                    let title = item.title.unwrap_or_default();
                    let link = item.link.unwrap_or_default();

                    let posted_on = item.pub_date.unwrap_or_default();

                    let timestamp = parse_date(&posted_on)?;

                    let job_post = mapped_detail(posted_on, timestamp, title, link, description)?;

                    let job_post = JobPostDB {
                        title: job_post.title,
                        link: job_post.link,
                        category: job_post.category,
                        detail: serde_json::to_value(job_post.detail)?,
                        posted_timestamp: job_post.posted_timestamp,
                        posted_on: string_to_datetime(&job_post.posted_on),
                    };
                    let num_row = RequestOperation::Insert(job_post).execute(pool).await;
                    Ok(())
                }
                None => println!("No description"),
            }
        });
    }
}
