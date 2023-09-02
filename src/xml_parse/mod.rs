mod html_parse;
mod mapped_detail;

use std::{
    collections::HashMap,
    io::{BufRead, Bytes},
};

use html_parse::get_detail;
use mapped_detail::mapped_detail;
use rss::Channel;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
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
                let title = item.title.unwrap_or_default();
                let link = item.link.unwrap_or_default();

                let job_post = mapped_detail(title, link, description)?;

                data.push(job_post);
            }
            None => continue,
        }
    }

    Ok(data)
}
