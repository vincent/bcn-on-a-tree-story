use std::{collections::BTreeMap, sync::Arc};

use crate::{prelude::W, utils::macros::map};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{thing, Array, Object, Value, Geometry},
    Datastore, Response, Session,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_sci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_es: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_cat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbor_id: Option<i64>,

    pub position: Geometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
    pub text: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Message> for Value {
    fn from(val: Message) -> Self {
        match val.id {
            Some(v) => map![
                    "id".into() => v.into(),
                    "text".into() => val.text.into(),
                    "completed".into() => val.completed.into(),
            ]
            .into(),
            None => map![
                "text".into() => val.text.into(),
                "completed".into() => val.completed.into()
            ]
            .into(),
        }
    }
}

impl Creatable for Message {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

pub trait Creatable: Into<Value> {}

#[derive(Clone)]
pub struct DB {
    pub ds: Arc<Datastore>,
    pub sesh: Session,
}

impl DB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, crate::error::Error> {
        let res = self.ds.execute(query, &self.sesh, vars, false).await?;
        Ok(res)
    }

    pub async fn add_message(&self, tree_id: String, text: String) -> Result<Object, crate::error::Error> {
        let sql = "CREATE message SET tree_id = $tree_id, text = $text, completed = false, created_at = time::now()";
        let vars: BTreeMap<String, Value> = map![
            "tree_id".into() => Value::Strand(tree_id.into()),
            "text".into() => Value::Strand(text.into()),
        ];
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_all_messages(&self, tree_id: String) -> Result<Vec<Object>, crate::error::Error> {
        let sql = "SELECT * FROM message WHERE tree_id = $tree_id ORDER BY created_at ASC;";

        let vars: BTreeMap<String, Value> = map![
            "tree_id".into() => Value::Strand(tree_id.into()),
        ];
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn get_all_trees_around(&self, lat: f32, long: f32) -> Result<Vec<Object>, crate::error::Error> {
        let sql = "SELECT * FROM trees WHERE geo::distance(position, $from) < 10;";

        let vars: BTreeMap<String, Value> = map![
            "from".into() => Value::Geometry((lat.into(), long.into()).into()),
            // "lat".into() => Value::Number(lat.into()),
            // "long".into() => Value::Number(long.into()),
        ];
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn delete_message(&self, id: String) -> Result<AffectedRows, crate::error::Error> {
        let sql = "Delete $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}
