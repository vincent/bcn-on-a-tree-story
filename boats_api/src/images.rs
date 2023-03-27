use reqwest::Client;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::io::ErrorKind;

#[derive(Deserialize)]
pub struct Pages {
    pub query: String,
    pub titles: Vec<String>,
    pub summaries: Vec<String>,
    pub urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct WikipediaResponse {
    query: Query,
}

#[derive(Debug, Deserialize)]
struct Query {
    pages: HashMap<String, Page>,
}

#[derive(Debug, Deserialize)]
struct Page {
    images: Vec<Image>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct Image {
    title: String,
}

async fn find_images_on_first_page(query: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let client = Client::new();

    let response: Pages = client
        .get("https://en.wikipedia.org/w/api.php")
        .query(&[
            ("action", "opensearch"),
            ("format", "json"),
            ("namespace", "0"),
            ("search", query),
            ("limit", "1"),
        ])
        .send()
        .await?
        .json()
        .await?;

    if response.titles.len() == 0 {
        return Err(Box::new(std::io::Error::new(ErrorKind::Other, "Unable to fetch all messages.")));
    }

    let title = response.titles[0].as_str();
    println!("found page {}", title);

    let response: WikipediaResponse = client
        .get("https://en.wikipedia.org/w/api.php")
        .query(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "images"),
            ("generator", "search"),
            ("gsrsearch", title),
            ("gsrlimit", "1"),
            ("imlimit", "max"),
        ])
        .send()
        .await?
        .json()
        .await?;

    let (_title, page) = response.query.pages.into_iter().next().unwrap();
    let image_titles: HashSet<String> = page.images
        .into_iter()
        .map(|image| image.title.clone())
        .filter(|name| !name.to_lowercase().contains("logo"))
        .take(3)
        .collect();

    Ok(image_titles)
}

pub async fn images_of(query: &str) -> Result<Vec<String>, std::io::Error> {
    let image_titles = find_images_on_first_page(&query)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch images."))?;

    let urls = image_titles
        .iter()
        .map(|title| {
            fmt::format(format_args!(
                "https://commons.wikimedia.org/wiki/Special:{}?width=500",
                title.replace("File:", "FilePath/")
            ))
        })
        .collect();

    return Ok(urls);
}
