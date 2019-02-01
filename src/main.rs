#![feature(proc_macro_hygiene, decl_macro)]
extern crate env_logger;
extern crate log;
#[macro_use]
extern crate rocket;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use serde_derive::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    url: String,
    read: bool,
}

mod routes {
    use rocket::http::Status;
    use std::result::Result;

    use rocket_contrib::json::Json;
    type Link = super::Link;

    #[post("/link", data = "<link>")]
    pub fn post(link: Json<Link>) -> Status {
        match super::save_link(&link) {
            Ok(_) => Status::NoContent,
            Err(_) => Status::InternalServerError,
        }
    }

    #[get("/link/<id>")]
    pub fn get_by_id(id: String) -> Status {
        println!("{:?}", id);
        //let link = get_link(id);

        Status::NoContent
    }

    #[get("/link")]
    pub fn get_all() -> Result<Json<Vec<Link>>, rocket::http::Status> {
        super::get_all_links()
            .map(|links| Json(links))
            .map_err(|_| Status::InternalServerError)
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::post, routes::get_by_id, routes::get_all],
        )
        .launch();
}

fn save_link(link: &Link) -> Result<()> {
    write!(get_tmp_file()?, "{},{}\n", link.url, link.read).map_err(|err| err.into())
}

fn get_all_links() -> Result<Vec<Link>> {
    parse_file(&get_tmp_file()?)
}

fn parse_file(file: &File) -> Result<Vec<Link>> {
    let mut link_vec: Vec<Link> = Vec::new();
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line?;

        let split = line.split(",").collect::<Vec<&str>>();

        let read: bool = split[1].parse::<bool>()?;

        let link = Link {
            url: split[0].to_string(),
            read,
        };
        link_vec.push(link);
    }

    Ok(link_vec)
}

fn get_tmp_file() -> Result<File> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("/tmp/foo.txt")
        .map_err(|err| err.into())
}
