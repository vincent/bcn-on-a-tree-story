use crate::models::*;
use reqwasm::{http::{Request}, Error};

const BASE_URL: &str = "/api";

pub async fn fetch_trees(lat: f64, long: f64) -> Result<Vec<Tree>, Error> {
    // let lat = 41.379368304896055;
    // let long = 2.1898975212208565;
    Request::get(&format!("{BASE_URL}/trees/{lat}/{long}/100"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn closest(lat: f64, long: f64) -> i32 {
    // let lat = 41.379368304896055;
    // let long = 2.1898975212208565;
    let response = Request::get(&format!("{BASE_URL}/near/{lat}/{long}"))
        .send()
        .await;
    
    match response {
        Ok(res) => {
            let payload = res.json().await; // could be `Error` or `Response` but only parses to `Response`
            match payload {
                Ok(j) => j,
                Err(_e) => 100000,
            }
        }
        Err(_e) => 100000,
    }
}

pub async fn fetch_messages(tree_id: &str) -> Result<Vec<Message>, Error> {
    Request::get(&format!("{BASE_URL}/messages/{tree_id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn create_message(tree_id: &str, title: &str) -> Result<Message, Error> {
    Request::post(&format!("{BASE_URL}/message/{tree_id}/{title}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn delete_message(id: String) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{BASE_URL}/message/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}
