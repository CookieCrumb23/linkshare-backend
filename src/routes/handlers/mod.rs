extern crate env_logger;
extern crate log;

use crate::routes::Link;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

mod io;

pub fn save_link(link: &Link) -> Result<()> {
    io::save_link(link)
}

pub fn get_link(id: &str) -> Option<Link> {
    let links = get_all_links();

    if links.is_err() {
        return None;
    }

    for link in links.unwrap() {
        if id == link.id {
            return Some(link);
        }
    }

    None
}

pub fn get_all_links() -> Result<Vec<Link>> {
        Ok(
            io::get_all_links()?
            .iter()
            .map(|entry| -> Link {
                let (key, (url, read)) = entry;

                Link{id: key.to_string(), url: url.to_string(), read: *read}
            }).collect::<Vec<Link>>()
        )
}
