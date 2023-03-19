use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Point {
    pub lat: f32,
    pub long: f32,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Tree {
    pub id: String,
    pub tree_id: String,
    pub name_sci: Option<String>,
    pub name_es: Option<String>,
    pub name_cat: Option<String>,
    pub space: Option<String>,
    pub district: Option<String>,
    pub neighbor: Option<String>,
    pub neighbor_id: Option<i64>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Message {
    pub id: String,
    pub tree_id: String,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

#[derive(Deserialize)]
pub struct RowId {
    pub id: String,
}
