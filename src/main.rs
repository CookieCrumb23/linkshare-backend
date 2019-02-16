#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::post, routes::get_by_id, routes::get_all, routes::set_read_state],
        )
        .launch();
}
