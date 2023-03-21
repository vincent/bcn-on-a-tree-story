#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};

use std::{io::ErrorKind, sync::Arc};
use surrealdb::{sql::Object, Datastore, Session};

use crate::db::{AffectedRows, DB};

use cors::*;

mod db;
mod error;
mod prelude;
mod utils;
mod cors;

#[get("/trees/<lat>/<long>")]
async fn get_all_trees(lat: f32, long: f32, db: &State<DB>) -> Result<Json<Vec<Object>>, std::io::Error> {
    let messages = db
        .get_all_trees_around(lat, long)
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
    let ds = Arc::new(Datastore::new("file:///database.surreal").await.unwrap());
    let sesh = Session::for_db("my_ns", "my_db");

    let db = DB { ds, sesh };

    rocket::build()
        .mount(
            "/",
            routes![get_all_trees, add_message, get_all_messages, delete_message],
        )
        .attach(CORS)
        .manage(db)
}
