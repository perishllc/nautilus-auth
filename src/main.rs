#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::{table, Insertable, Queryable};
use rocket::{fairing::AdHoc, serde::json::Json, State};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use crate::diesel::RunQueryDsl;

table! {
    seeds (identifier) {
        identifier -> Varchar,
        encrypted_seed -> Varchar,
    }
}

#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "seeds"]
struct SeedBackup {
    identifier: String,
    encrypted_seed: String,
}

#[derive(Deserialize)]
struct Config {
    name: String,
    version: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/config")]
fn custom(config: &State<Config>) -> String {
    format!("{} : version: {}", config.name, config.version)
}

#[get("/<identifier>")]
fn get_seed_backup(connection: Db, identifier: String) -> Json<SeedBackup> {
    Json(SeedBackup {
        identifier: "some_identifier".to_string(),
        encrypted_seed: "some_seed".to_string(),
    })


    // SELECT * FROM seeds WHERE identifier = identifier

    

    // connection
    // .run(move |c| {
    //     diesel::QueryResult(seeds::table)
    //         .values(&seed_backup.into_inner())
    //         .get_result(c)
    // })
    // .await
}

#[post("/", data = "<seed_backup>")]
async fn create_seed_backup(connection: Db, seed_backup: Json<SeedBackup>) -> Json<SeedBackup> {
  
    connection
        .run(move |c| {
            diesel::insert_into(seeds::table)
                .values(&seed_backup.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")

    // println!("seed_backup: {:?}", seed_backup);

    // Json(SeedBackup {
    //     identifier: "some_identifier".to_string(),
    //     encrypted_seed: "some_seed".to_string(),
    // })
  
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket
        .attach(Db::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, custom])
        .mount("/seed-backup", routes![get_seed_backup, create_seed_backup])
}
