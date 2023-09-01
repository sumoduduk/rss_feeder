#![allow(unused_imports)]
mod html_parse;
mod xml_parse;

pub use html_parse::get_detail;
pub use xml_parse::parse_xml;

use reqwest::Client;
use rss::Channel;
use scraper::{Html, Selector};
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    //load from dot_envy
    // let file = Client::new().get(uri).send().await?.bytes().await?;
    //
    // let bye = &file[..];

    // let channel = Channel::read_from(&response[..]).unwrap();

    let path = Path::new("./example/rss.xml");

    let file = File::open(path)?;

    let reader = BufReader::new(&file);
    // let channel = Channel::read_from(BufReader::new(&file))?;

    let item = parse_xml(reader)?;

    let target_path = Path::new("./example/job_posts1.json");

    let json_str = serde_json::to_string_pretty(&item).unwrap();
    fs::write(target_path, json_str).expect("Unable to write file");
    Ok(())
}
