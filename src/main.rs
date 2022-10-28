#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use crate::diesel::RunQueryDsl;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::{table, Insertable, Queryable};
use rocket::response::status::NotFound;
use rocket::{fairing::AdHoc, serde::json::Json, State};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

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

// #[get("/<identifier>")]
// fn get_seed_backup(connection: Db, identifier: String) -> Json<SeedBackup> {
//     println!("identifier: {}", identifier);

//     // Json(SeedBackup {
//     //     identifier: "some_identifier".to_string(),
//     //     encrypted_seed: "some_seed".to_string(),
//     // })
//     // SELECT * FROM seeds WHERE identifier = identifier

//     // let seed_backup = seeds::table
//     //     .filter(seeds::identifier.eq(identifier));

//     // Json(seed_backup.first::<SeedBackup>(&connection).unwrap())

//     let res = connection
//     .run(|c| seeds::table.load(c))
//     .await
//     .map(Json)
//     .expect("Failed to fetch blog posts");

//     res.filter

//     // let results = seeds::table.load(&connection).expect("Error loading seeds");

//     // println!("Displaying {} posts", results);

//     // connection
//     // .run(move |c| {
//     //     diesel::QueryResult(seeds::table)
//     //         .values(&seed_backup.into_inner())
//     //         .get_result(c)
//     // })
//     // .await

//     // Json(SeedBackup {
//     //     identifier: "some_identifier".to_string(),
//     //     encrypted_seed: "some_seed".to_string(),
//     // })
// }

#[get("/<id>")]
async fn get_seed_backup(id: String, connection: Db) -> Result<Json<SeedBackup>, NotFound<String>> {
    println!("identifier: {}", id);

    let res = connection
        .run(|c| {
            seeds::table
                .filter(seeds::identifier.eq(id))
                .first::<SeedBackup>(c)
        })
        .await
        .map(Json);
    if res.is_ok() {
        return Ok(res.unwrap());
    }

    return Err(NotFound(format!("Could not find seed")));
}

#[post("/", data = "<seed_backup>")]
async fn create_seed_backup(connection: Db, seed_backup: Json<SeedBackup>) -> Result<Json<SeedBackup>, NotFound<String>> {
    let res = connection
        .run(move |c| {
            diesel::insert_into(seeds::table)
                .values(&seed_backup.into_inner())
                .get_result(c)
        })
        .await
        .map(Json);

    // check if res is ok, if not return error

    if res.is_ok() {
        return Ok(res.unwrap());
    }

    return Err(NotFound(format!("Something went wrong")));
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
