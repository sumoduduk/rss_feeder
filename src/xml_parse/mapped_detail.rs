use super::{html_parse::get_detail, JobPost};

pub fn mapped_detail(title_raw: String, link_raw: String, desc: String) -> eyre::Result<JobPost> {
    let links: Vec<_> = link_raw.split("?").collect();

    let details = get_detail(&desc)?;

    let job_post = JobPost {
        title: title_raw,
        link: links[0].to_owned(),
        detail: details,
    };

    Ok(job_post)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_1_get_detail() {
        let title_test = "Title 01".to_string();
        let link_test = "https://linktest.com".to_string();

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

        let job_post = JobPost {
            title: "Title 01".to_string(),
            link: "https://linktest.com".to_string(),
            detail: expected1,
        };

        let mapped_detail = mapped_detail(title_test, link_test, test1.to_string()).unwrap();

        assert_eq!(job_post, mapped_detail);
    }

    #[test]
    fn test_2_get_detail() {
        let title_test = "Title 02".to_string();
        let link_test = "https://linktest2.com".to_string();

        let test2 = "New logo design needed for a tech startup. We are looking for a modern and clean design.<br /><br /><b>Hourly Range</b>: $15.00-$25.00\n\n<br /><b>Posted On</b>: September 02, 2023 03:17 UTC<br /><b>Category</b>: Graphic Design<br /><b>Skills</b>:Logo Design,     Branding,     Adobe Illustrator,     Adobe Photoshop    \n<br /><b>Country</b>: United Kingdom\n<br /><a href=\"https://www.upwork.com/jobs/Logo-Design_%7E014431774d3a21a1a3?source=rss\">click to apply</a>\n";
        let mut expected2 = HashMap::new();
        expected2.insert("Hourly Range".to_string(), "$15.00-$25.00".to_string());
        expected2.insert(
            "Posted On".to_string(),
            "September 02, 2023 03:17 UTC".to_string(),
        );
        expected2.insert("Category".to_string(), "Graphic Design".to_string());
        expected2.insert(
            "Skills".to_string(),
            "Logo Design, Branding, Adobe Illustrator, Adobe Photoshop".to_string(),
        );
        expected2.insert("Country".to_string(), "United Kingdom".to_string());

        let job_post = JobPost {
            title: "Title 02".to_string(),
            link: "https://linktest2.com".to_string(),
            detail: expected2,
        };

        let mapped_detail = mapped_detail(title_test, link_test, test2.to_string()).unwrap();

        assert_eq!(job_post, mapped_detail);
    }

    #[test]
    fn test_3_get_detail() {
        let title_test = "Title 03".to_string();
        let link_test = "https://linktest3.com".to_string();

        let test3 = "We need a content writer for our blog posts. The topics will be mainly about tech and startups.<br /><br /><b>Hourly Range</b>: $20.00-$30.00\n\n<br /><b>Posted On</b>: September 03, 2023 04:17 UTC<br /><b>Category</b>: Writing<br /><b>Skills</b>:Content Writing,     Blog Writing,     Technical Writing,     SEO Writing   \n<br /><b>Country</b>: Canada\n<br /><a href=\"https://www.upwork.com/jobs/Content-Writer_%7E014431774d3a21a1a4?source=rss\">click to apply</a>\n";
        let mut expected3 = HashMap::new();
        expected3.insert("Hourly Range".to_string(), "$20.00-$30.00".to_string());
        expected3.insert(
            "Posted On".to_string(),
            "September 03, 2023 04:17 UTC".to_string(),
        );
        expected3.insert("Category".to_string(), "Writing".to_string());
        expected3.insert(
            "Skills".to_string(),
            "Content Writing, Blog Writing, Technical Writing, SEO Writing".to_string(),
        );
        expected3.insert("Country".to_string(), "Canada".to_string());

        let job_post = JobPost {
            title: "Title 03".to_string(),
            link: "https://linktest3.com".to_string(),
            detail: expected3,
        };

        let mapped_detail = mapped_detail(title_test, link_test, test3.to_string()).unwrap();

        assert_eq!(job_post, mapped_detail);
    }

    #[test]
    fn test_4_get_detail() {
        let title_test = "Title 04".to_string();
        let link_test = "https://linktest4.com".to_string();

        let test4 = "Looking for a web developer to build a responsive website for our company.<br /><br /><b>Hourly Range</b>: $25.00-$35.00\n\n<br /><b>Posted On</b>: September 04, 2023 05:17 UTC<br /><b>Category</b>: Web Development<br /><b>Skills</b>:HTML5,     CSS3,     JavaScript,     Responsive Web Design   \n<br /><b>Country</b>: Australia\n<br /><a href=\"https://www.upwork.com/jobs/Web-Developer_%7E014431774d3a21a1a5?source=rss\">click to apply</a>\n";
        let mut expected4 = HashMap::new();
        expected4.insert("Hourly Range".to_string(), "$25.00-$35.00".to_string());
        expected4.insert(
            "Posted On".to_string(),
            "September 04, 2023 05:17 UTC".to_string(),
        );
        expected4.insert("Category".to_string(), "Web Development".to_string());
        expected4.insert(
            "Skills".to_string(),
            "HTML5, CSS3, JavaScript, Responsive Web Design".to_string(),
        );
        expected4.insert("Country".to_string(), "Australia".to_string());

        let job_post = JobPost {
            title: "Title 04".to_string(),
            link: "https://linktest4.com".to_string(),
            detail: expected4,
        };

        let mapped_detail = mapped_detail(title_test, link_test, test4.to_string()).unwrap();

        assert_eq!(job_post, mapped_detail);
    }

    #[test]
    fn test_5_get_detail() {
        let title_test = "Title 05".to_string();
        let link_test = "https://linktest5.com".to_string();

        let test5 = "We are hiring a social media manager to handle our company's social media accounts.<br /><br /><b>Hourly Range</b>: $30.00-$40.00\n\n<br /><b>Posted On</b>: September 05, 2023 06:17 UTC<br /><b>Category</b>: Social Media<br /><b>Skills</b>:Social Media Management,     Content Creation,     Instagram Marketing,     Facebook Marketing   \n<br /><b>Country</b>: Germany\n<br /><a href=\"https://www.upwork.com/jobs/Social-Media-Manager_%7E014431774d3a21a1a6?source=rss\">click to apply</a>\n";
        let mut expected5 = HashMap::new();
        expected5.insert("Hourly Range".to_string(), "$30.00-$40.00".to_string());
        expected5.insert(
            "Posted On".to_string(),
            "September 05, 2023 06:17 UTC".to_string(),
        );
        expected5.insert("Category".to_string(), "Social Media".to_string());
        expected5.insert(
            "Skills".to_string(),
            "Social Media Management, Content Creation, Instagram Marketing, Facebook Marketing"
                .to_string(),
        );
        expected5.insert("Country".to_string(), "Germany".to_string());

        let job_post = JobPost {
            title: "Title 05".to_string(),
            link: "https://linktest5.com".to_string(),
            detail: expected5,
        };

        let mapped_detail = mapped_detail(title_test, link_test, test5.to_string()).unwrap();

        assert_eq!(job_post, mapped_detail);
    }
}
