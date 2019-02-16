extern crate md5;

use serde_derive::{Deserialize, Serialize};
use md5::{Md5, Digest};
use rocket::http::Status;
use std::result::Result;
use std::collections::HashSet;

use rocket_contrib::json::Json;

mod handlers;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Link {
    pub id: String,
    pub url: String,
    pub read: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    url: String
}


#[post("/link", data = "<url>")]
pub fn post(url: Json<Url>) -> Result<Json<Link>, Status> {

    let id = generate_id(&url.url);

    let link = Link{url: url.url.to_owned(), read: false, id};

    match handlers::save_link(&link) {
        Ok(_) => Ok(Json(link)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/link/<id>")]
pub fn get_by_id(id: String) -> Result<Json<Link>, Status> {
    handlers::get_link(&id)
        .map(Json)
        .ok_or(Status::NotFound)
}

#[get("/link")]
pub fn get_all() -> Result<Json<HashSet<Link>>, Status> {
    handlers::get_all_links()
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[put("/link/<id>/read")]
pub fn set_read_state(id: String) -> Status {
    println!("{:?}", id);
    Status::NoContent
}

fn generate_id(url: &str) -> String {
    format!(
        "{:x}",
        Md5::new()
            .chain(url)
            .result())
        [0..8]
        .to_owned()
}
