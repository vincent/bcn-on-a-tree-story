use std::env;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use surrealdb::{
    sql::{Value as DbValue},
    Datastore, Error as DbError, Session,
};

#[tokio::main]
async fn main() -> Result<(), DbError> {
    let _args: Vec<String> = env::args().collect();

    let ds = Datastore::new("file:///database/surreal/surreal").await?;
    let ses = Session::for_db("my_ns", "my_db");
    let sql = "SELECT * FROM trees LIMIT 100;";

    let res = ds.execute(sql, &ses, None, false).await?;
    for row in res.into_iter() {
        if let Ok(r) = row.result {
            println!("{:#?}", r);
        }
    }

    let res = ds.execute("SELECT count() FROM trees GROUP BY ALL;", &ses, None, false).await?;
    let first_res = res.into_iter().next().expect("Did not get a response");
    println!("====>> {}", first_res.result?.first().single());

    Ok(())
}
