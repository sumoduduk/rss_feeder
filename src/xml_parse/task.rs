use std::{thread, time::Duration};

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
        thread::sleep(Duration::from_secs(1));

        let pool = pool.clone();

        rt::spawn(async move {
            let desc = item.description;

            match desc {
                Some(description) => {
                    let mut inserted_not_inserted = (0u8, 0u8);

                    let title = item.title.unwrap_or_default();
                    let link = item.link.unwrap_or_default();

                    let posted_on = item.pub_date.unwrap_or_default();

                    let timestamp = parse_date(&posted_on);

                    if let Ok(timestamp_res) = timestamp {
                        let job_post_res =
                            mapped_detail(posted_on, timestamp_res, title, link, description);

                        if let Ok(job_post) = job_post_res {
                            let detail_res = serde_json::to_value(job_post.detail);
                            let posted_res = string_to_datetime(&job_post.posted_on);

                            match (detail_res, posted_res) {
                                (Ok(detail), Ok(posted)) => {
                                    let job_post = JobPostDB {
                                        title: job_post.title,
                                        link: job_post.link,
                                        category: job_post.category,
                                        detail,
                                        posted_timestamp: job_post.posted_timestamp,
                                        posted_on: posted,
                                    };
                                    let num_row =
                                        RequestOperation::Insert(job_post).execute(&pool).await;

                                    match num_row {
                                        Ok(_) => inserted_not_inserted.0 += 1,
                                        Err(_) => inserted_not_inserted.1 += 1,
                                    }
                                }
                                (_, _) => println!("Shit happen"),
                            }
                        }
                    }
                }
                None => println!("No description"),
            }
        });
    }
}
