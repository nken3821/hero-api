#[macro_use] extern crate rocket;
mod hero;

use rocket::serde::json::Json;
use serde_json::{json, Value};
use hero::Hero;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}
#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "hero 1",
        "hero 2",
    ]))
}
#[put("/<_id>", data = "<hero>")]
fn update(_id: i32, hero: Json<Hero>) -> Json<Hero> {
    hero
}
#[delete("/<_id>")]
fn delete(_id: i32) -> Json<Value> {
    Json(json!({
        "status": "ok"
    }))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch().await.unwrap();
}
