use std::{
    collections::HashMap,
    io::{BufRead, Bytes},
};

use rss::Channel;
use serde::Serialize;

use crate::get_detail;

#[derive(Serialize)]

pub struct JobPost {
    title: String,
    link: String,
    detail: HashMap<String, String>,
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
                let link_raw = item.link.unwrap_or_default();
                let links: Vec<_> = link_raw.split("?").collect();

                let details = get_detail(&description)?;

                let job_post = JobPost {
                    title: item.title.unwrap_or_default(),
                    link: links[0].to_owned(),
                    detail: details,
                };

                data.push(job_post);
            }
            None => continue,
        }
    }

    Ok(data)
}
