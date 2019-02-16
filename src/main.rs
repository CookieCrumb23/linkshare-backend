#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate env_logger;
extern crate log;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Link {
    id: String,
    url: String,
    read: bool,
}

mod routes {
    extern crate md5;

    use serde_derive::{Deserialize, Serialize};
    use md5::{Md5, Digest};
    use rocket::http::Status;
    use std::result::Result;
    use std::collections::HashSet;

    use rocket_contrib::json::Json;

    type Link = super::Link;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Url {
        url: String
    }


    #[post("/link", data = "<url>")]
    pub fn post(url: Json<Url>) -> Result<Json<Link>, Status> {

        let id = generate_id(&url.url);

        let link = Link{url: url.url.to_owned(), read: false, id};

        match super::save_link(&link) {
            Ok(_) => Ok(Json(link)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    #[get("/link/<id>")]
    pub fn get_by_id(id: String) -> Result<Json<Link>, Status> {
        super::get_link(&id)
            .map(Json)
            .ok_or(Status::NotFound)
    }

    #[get("/link")]
    pub fn get_all() -> Result<Json<HashSet<Link>>, Status> {
        super::get_all_links()
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
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::post, routes::get_by_id, routes::get_all, routes::set_read_state],
        )
        .launch();
}

fn save_link(link: &Link) -> Result<()> {
    writeln!(get_tmp_file()?, "{},{},{}", link.id, link.url, link.read).map_err(Into::into)
}

fn get_all_links() -> Result<HashSet<Link>> {
    parse_file(&get_tmp_file()?)
}

fn parse_file(file: &File) -> Result<HashSet<Link>> {
    let mut link_set: HashSet<Link> = HashSet::new();
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line?;

        let split = line.split(',').collect::<Vec<&str>>();

        let read: bool = split[2].parse::<bool>()?;

        let link = Link {
            id: split[0].to_string(),
            url: split[1].to_string(),
            read,
        };

        link_set.insert(link);
    }

    Ok(link_set)
}

fn get_link(id: &str) -> Option<Link> {
    let links = get_all_links();

    if links.is_err() {
        return None
    }

    for link in links.unwrap() {
        if id == link.id {
            return Some(link)
        }
    };

    None
}

fn get_tmp_file() -> Result<File> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("/tmp/foo.txt")
        .map_err(Into::into)
}
