#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State, response::Redirect};

use std::{io::ErrorKind, sync::Arc};
use url::Url;
use surrealdb::{sql::Object, Datastore, Session};

use crate::db::{AffectedRows, DB};

use cors::*;

mod ai;
mod cors;
mod db;
mod error;
mod images;
mod prelude;
mod utils;

#[get("/near/<lat>/<long>")]
async fn get_proximity(lat: f32, long: f32, db: &State<DB>) -> Result<Json<Object>, std::io::Error> {
    let result = db
        .get_proximity(lat, long)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Unable to fetch proximity: {}", e)))?;

    Ok(Json(result))
}

#[get("/trees/<lat>/<long>/<distance>")]
async fn get_all_trees(lat: f32, long: f32, distance: i32, db: &State<DB>) -> Result<Json<Vec<Object>>, std::io::Error> {
    let messages = db
        .get_all_trees_around(lat, long, distance)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Unable to fetch trees: {}", e)))?;

    Ok(Json(messages))
}

#[post("/message/<tree_id>/<title>")]
async fn add_message(tree_id: String, title: String, db: &State<DB>) -> Result<Json<Object>, std::io::Error> {
    let message = db
        .add_message(tree_id, title)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create message."))?;

    Ok(Json(message))
}

#[get("/messages/<tree_id>")]
async fn get_all_messages(tree_id: String, db: &State<DB>) -> Result<Json<Vec<Object>>, std::io::Error> {
    let messages = db
        .get_all_messages(tree_id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch all messages."))?;

    Ok(Json(messages))
}

#[get("/img/<sci_name>")]
async fn get_tree_picture(sci_name: String, db: &State<DB>) -> Redirect {
    let url = db
        .image_of(sci_name)
        .await
        .unwrap();

    let parsed_url = Url::parse(url.as_str()).unwrap();
    Redirect::permanent(parsed_url.to_string())
}

#[get("/txt/<lang>/<sci_name>/<nei_name>")]
async fn get_tree_text(lang: String, sci_name: String, nei_name: String, db: &State<DB>) -> Result<Json<String>, std::io::Error> {
    let txt = db
        .prompt_of(lang, sci_name, nei_name)
        .await
        .unwrap_or("I have nothing to say yet".to_string());

    Ok(Json(txt))
}

#[delete("/message/<id>")]
async fn delete_message(id: String, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let affected_rows = db
        .delete_message(id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to delete message."))?;

    Ok(Json(affected_rows))
}

#[launch]
async fn rocket() -> _ {
    let ds = Arc::new(Datastore::new("file:///database/surreal/surreal").await.unwrap());
    let sesh = Session::for_db("my_ns", "my_db");

    let db = DB { ds, sesh };

    rocket::build()
        .mount(
            "/",
            routes![get_proximity, get_all_trees, get_tree_picture, get_tree_text, add_message, get_all_messages, delete_message],
        )
        .attach(CORS)
        .manage(db)
}
