use std::collections::{HashSet, HashMap};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

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
        return Err(Box::new(MyError::new("could not find images")))
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
    let image_titles: HashSet<String> = page.images[0..3].into_iter().map(|image| image.title.clone()).collect();

    Ok(image_titles)
}

#[tokio::main]
async fn main() {
    let query = "Rust programming language".to_owned();
    let image_titles = find_images_on_first_page(&query).await.expect("results");

    let images_urls: Vec<String> = image_titles
        .iter()
        .map(|title| fmt::format(format_args!("https://commons.wikimedia.org/wiki/Special:{}?width=500", title.replace("File:", "FilePath/"))))
        .collect();

    for url in images_urls {
        println!("{}", url);
    }
}
