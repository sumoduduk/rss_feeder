use eyre::eyre;
use std::collections::HashMap;

use scraper::{Html, Selector};

pub fn get_detail(description: &str) -> eyre::Result<HashMap<String, String>> {
    let doc = Html::parse_document(description);

    let selector = Selector::parse("b").unwrap();

    let elements = doc.select(&selector);
    let mut mapped = HashMap::new();

    for element in elements {
        let key = element.inner_html();

        let value: String = element
            .next_sibling()
            .ok_or_else(|| eyre!("html parses dont have next sibling"))?
            .value()
            .as_text()
            .ok_or_else(|| eyre!("html parser are not a text"))?
            .to_string();

        if key == "Posted On".to_owned() {
            println!("{}", &value);
        }

        let value: Vec<&str> = value
            .trim_start_matches(":")
            .trim()
            .split_whitespace()
            .collect();
        let value = value.join(" ");

        mapped.insert(key, value);
    }
    Ok(mapped)
}

#[test]
fn test_get_detail() {
    let test1 = "Picture needs to be designed for the HERO page. Background needs to be changed and some design adjustments<br /><br /><b>Hourly Range</b>: $10.00-$20.00\n\n<br /><b>Posted On</b>: September 01, 2023 02:17 UTC<br /><b>Category</b>: Web Design<br /><b>Skills</b>:Web Design,     Graphic Design,     Illustration,     Website,     Landing Page,     Blog,     Website Asset    \n<br /><b>Skills</b>:        Web Design,                     Graphic Design,                     Illustration,                     Website,                     Landing Page,                     Blog,                     Website Asset            <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Website-Hero-Page_%7E014431774d3a21a1a2?source=rss\">click to apply</a>\n";
    let mut expected1 = HashMap::new();
    expected1.insert("Hourly Range".to_string(), "$10.00-$20.00".to_string());
    expected1.insert(
        "Posted On".to_string(),
        "September 01, 2023 02:17 UTC".to_string(),
    );
    expected1.insert("Category".to_string(), "Web Design".to_string());
    expected1.insert(
        "Skills".to_string(),
        "Web Design, Graphic Design, Illustration, Website, Landing Page, Blog, Website Asset"
            .to_string(),
    );
    expected1.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test1).unwrap(), expected1);
}

#[test]
fn test_get_detail_2() {
    let test2 = "I have built a simple 4 page website in Word Press using Elementor and it looks great on a laptop but the mobile version formatting is a mess. I need someone to fix the formatting for mobile without changing the regular site appearance. Site is here: http://wordpress.37minutes.co/<br /><br /><b>Budget</b>: $22\n<br /><b>Posted On</b>: September 01, 2023 02:06 UTC<br /><b>Category</b>: Web Design<br /><b>Skills</b>:Elementor,     WordPress,     Web Design    \n<br /><b>Skills</b>:        Elementor,                     WordPress,                     Web Design            <br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Wordpress-Elementor-Mobile-Friendly-Site-Help-Needed_%7E01a306c7ae90e97dfc?source=rss\">click to apply</a>\n";
    let mut expected2 = HashMap::new();
    expected2.insert("Budget".to_string(), "$22".to_string());
    expected2.insert(
        "Posted On".to_string(),
        "September 01, 2023 02:06 UTC".to_string(),
    );
    expected2.insert("Category".to_string(), "Web Design".to_string());
    expected2.insert(
        "Skills".to_string(),
        "Elementor, WordPress, Web Design".to_string(),
    );
    expected2.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test2).unwrap(), expected2);
}
