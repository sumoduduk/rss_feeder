mod html_parse;
mod mapped_detail;

use std::{
    collections::HashMap,
    io::{BufRead, Bytes},
};

use html_parse::get_detail;
use mapped_detail::mapped_detail;
use rss::Channel;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::parse_date;

#[derive(FromRow, Debug, Serialize, Deserialize, PartialEq)]
pub struct JobPost {
    pub title: String,
    pub link: String,
    pub detail: HashMap<String, String>,
    pub posted_on: String,
    pub posted_timestamp: i64,
}

pub fn parse_xml<R>(reader: R) -> eyre::Result<Vec<JobPost>>
where
    R: BufRead,
{
    let channel = Channel::read_from(reader)?;

    let items = channel.items;
    let len = items.len();

    let mut data: Vec<JobPost> = Vec::with_capacity(len);

    for item in items {
        let desc = item.description;

        match desc {
            Some(description) => {
                let title = item.title.unwrap_or_default();
                let link = item.link.unwrap_or_default();

                let posted_on = item.pub_date.unwrap_or_default();

                let timestamp = parse_date(&posted_on)?;

                let job_post = mapped_detail(posted_on, timestamp, title, link, description);

                match job_post {
                    Ok(job_data) => data.push(job_data),
                    Err(_) => continue,
                }
            }
            None => continue,
        }
    }

    Ok(data)
}
