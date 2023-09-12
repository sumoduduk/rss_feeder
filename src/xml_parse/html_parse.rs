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

        if key == *"Posted On".to_owned() {
            continue;
        }

        let value: String = element
            .next_sibling()
            .ok_or_else(|| eyre!("html parse: dont have next sibling"))?
            .value()
            .as_text()
            .ok_or_else(|| eyre!("html parse: are not a text"))?
            .to_string();

        let value: Vec<&str> = value.trim_start_matches(':').split_whitespace().collect();
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
    expected2.insert("Category".to_string(), "Web Design".to_string());
    expected2.insert(
        "Skills".to_string(),
        "Elementor, WordPress, Web Design".to_string(),
    );
    expected2.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test2).unwrap(), expected2);
}

#[test]
fn test_get_detail_3() {
    let test3 = "We need a logo for our new company. We are a tech startup focused on AI solutions.<br /><br /><b>Hourly Range</b>: $15.00-$25.00\n\n<br /><b>Posted On</b>: September 01, 2023 03:17 UTC<br /><b>Category</b>: Graphic Design<br /><b>Skills</b>:Logo Design,     Graphic Design,     Branding    \n<br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Logo-Design_%7E014431774d3a21a1a3?source=rss\">click to apply</a>\n";
    let mut expected3 = HashMap::new();
    expected3.insert("Hourly Range".to_string(), "$15.00-$25.00".to_string());
    expected3.insert("Category".to_string(), "Graphic Design".to_string());
    expected3.insert(
        "Skills".to_string(),
        "Logo Design, Graphic Design, Branding".to_string(),
    );
    expected3.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test3).unwrap(), expected3);
}

#[test]
fn test_get_detail_4() {
    let test4 = "Looking for a content writer for our blog. Must be familiar with the tech industry.<br /><br /><b>Budget</b>: $200\n<br /><b>Posted On</b>: September 01, 2023 04:06 UTC<br /><b>Category</b>: Writing<br /><b>Skills</b>:Content Writing,     Blog Writing,     Tech Writing    \n<br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Content-Writer-Needed_%7E01a306c7ae90e97dfd?source=rss\">click to apply</a>\n";
    let mut expected4 = HashMap::new();
    expected4.insert("Budget".to_string(), "$200".to_string());
    expected4.insert("Category".to_string(), "Writing".to_string());
    expected4.insert(
        "Skills".to_string(),
        "Content Writing, Blog Writing, Tech Writing".to_string(),
    );
    expected4.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test4).unwrap(), expected4);
}

#[test]
fn test_get_detail_5() {
    let test5 = "We are looking for a web developer to build our company website.<br /><br /><b>Budget</b>: $5000\n<br /><b>Posted On</b>: September 01, 2023 05:06 UTC<br /><b>Category</b>: Web Development<br /><b>Skills</b>:HTML,     CSS,     JavaScript,     Web Development   \n<br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Web-Developer-Needed_%7E01a306c7ae90e97dfe?source=rss\">click to apply</a>\n";
    let mut expected5 = HashMap::new();
    expected5.insert("Budget".to_string(), "$5000".to_string());
    expected5.insert("Category".to_string(), "Web Development".to_string());
    expected5.insert(
        "Skills".to_string(),
        "HTML, CSS, JavaScript, Web Development".to_string(),
    );
    expected5.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test5).unwrap(), expected5);
}

#[test]
fn test_get_detail_6() {
    let test6 = "Need a mobile app developer for a new project. Must have experience with React Native.<br /><br /><b>Budget</b>: $3000\n<br /><b>Posted On</b>: September 01, 2023 06:06 UTC<br /><b>Category</b>: Mobile Development<br /><b>Skills</b>:React Native,     Mobile App Development   \n<br /><b>Country</b>: United States\n<br /><a href=\"https://www.upwork.com/jobs/Mobile-App-Developer-Needed_%7E01a306c7ae90e97dff?source=rss\">click to apply</a>\n";
    let mut expected6 = HashMap::new();
    expected6.insert("Budget".to_string(), "$3000".to_string());
    expected6.insert("Category".to_string(), "Mobile Development".to_string());
    expected6.insert(
        "Skills".to_string(),
        "React Native, Mobile App Development".to_string(),
    );
    expected6.insert("Country".to_string(), "United States".to_string());
    assert_eq!(get_detail(test6).unwrap(), expected6);
}
