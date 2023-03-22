use std::{env, collections::BTreeMap, error::Error, fs::File, io::BufReader, path::Path};

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use surrealdb::{
    sql::{Value as DbValue},
    Datastore, Error as DbError, Session,
};

#[derive(Deserialize, Debug)]
struct Row {
    codi: String,
    #[serde(deserialize_with = "de_to_float")]
    latitud: f32,
    #[serde(deserialize_with = "de_to_float")]
    longitud: f32,
    nom_cientific: String,
    espai_verd: Option<String>,
    // adreca: Option<String>,
    // catalogacio: Option<String>,
    nom_districte: Option<String>,
    // tipus_element: Option<String>,
    nom_barri: Option<String>,
    // geom: Option<String>,
    nom_catala: Option<String>,
    // #[serde(deserialize_with = "de_to_float")]
    // x_etrs89: f32,
    // #[serde(deserialize_with = "de_to_float")]
    // y_etrs89: f32,
    // data_plantacio: Option<String>,
    #[serde(deserialize_with = "de_to_int")]
    codi_barri: i64,
    nom_castella: Option<String>,
    // codi_districte: Option<String>,
    // tipus_reg: Option<String>,
    // categoria_arbrat: Option<String>,
    // cat_especie_id: i64,
    // tipus_aigua: Option<String>,
}

macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
}

fn de_to_float<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f32,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

fn de_to_int<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? as i64,
        _ => 0,
    })
}

fn read_payload_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Row>, Box<dyn Error>> {
    // Open file in RO mode with buffer
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file
    let u: Vec<Row> = serde_json::from_reader(reader)?;

    Ok(u)
}

#[tokio::main]
async fn main() -> Result<(), DbError> {
    let args: Vec<String> = env::args().collect();
    let dbpath = &args[1];

    let ds = Datastore::new("file:///database/surreal").await?;
    let ses = Session::for_db("my_ns", "my_db");

    ds.execute("REMOVE TABLE trees;", &ses, None, false).await?;

    let payload: Vec<Row> =
        read_payload_from_file(dbpath).unwrap();
    for line in payload.iter() {
        let sql = "CREATE trees SET tree_id = $tree_id, position = $pos, name_sci = $name_sci, name_es = $name_es, name_cat = $name_cat, space = $space, district = $district, neighbor = $neighbor, neighbor_id = $neighbor_id;";
        let vars: BTreeMap<String, DbValue> = map![
            "tree_id".into()    => DbValue::Strand(line.codi.to_owned().into()),
            "pos".into()        => DbValue::Geometry((line.latitud.into(), line.longitud.into()).into()),
            "name_sci".into()   => DbValue::Strand(line.nom_cientific.to_owned().into()),
            "name_es".into()    => DbValue::Strand(line.nom_castella.clone().unwrap_or_default().into()),
            "name_cat".into()   => DbValue::Strand(line.nom_catala.clone().unwrap_or_default().into()),
            "space".into()      => DbValue::Strand(line.espai_verd.clone().unwrap_or_default().into()),
            "district".into()   => DbValue::Strand(line.nom_districte.clone().unwrap_or_default().into()),
            "neighbor".into()   => DbValue::Strand(line.nom_barri.clone().unwrap_or_default().into()),
            "neighbor_id".into()=> DbValue::Number(line.codi_barri.into())
        ];

        let res = ds.execute(sql, &ses, Some(vars), false).await?;
        let first_res = res.into_iter().next().expect("Did not get a response");
        println!("insert {}", first_res.result?.first().single());
    }

    let res = ds.execute("SELECT count() FROM trees GROUP BY ALL;", &ses, None, false).await?;
    let first_res = res.into_iter().next().expect("Did not get a response");
    println!("====>> {}", first_res.result?.first().single());

    Ok(())
}
