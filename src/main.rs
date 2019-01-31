#![feature(proc_macro_hygiene, decl_macro)]

extern crate env_logger;
extern crate log;
#[macro_use]
extern crate rocket;

use std::fs::{File, OpenOptions};
use std::io::{Result, Write};

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Link {
	url: String,
	read: bool,
}

#[post("/link", data = "<link>")]
fn post(link: Json<Link>) -> Status {
	println!("{:?}", link);
	let file = get_tmp_file();

	let mut foo = match file {
		Ok(t) => t,
		Err(e) => panic!(e)
	};

	let result = write!(foo, "{:?},{:?}", link.url, link.read);

	match result {
		Ok(_) => Status::NoContent,
		Err(_) => Status::InternalServerError
	}
}

fn get_tmp_file() -> Result<File> {
	OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open("/tmp/foo.txt")
}

fn main() {
	rocket::ignite().mount("/", routes![post]).launch();
}
