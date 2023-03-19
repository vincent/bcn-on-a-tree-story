use crate::models::*;
use reqwasm::{http::Request, Error};

const BASE_URL: &str = "http://localhost:8080";

pub async fn fetch_trees(lat: f64, long: f64) -> Result<Vec<Tree>, Error> {
    Request::get(&format!("{BASE_URL}/trees/41.4379546/2.1657465"))
        .send()
        .await
        .unwrap()
        .json()
        .await
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
