use crate::routes::Link;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn get_tmp_file() -> Result<File> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("/tmp/foo.txt")
        .map_err(Into::into)
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

pub fn save_link(link: &Link) -> Result<()> {
    writeln!(get_tmp_file()?, "{},{},{}", link.id, link.url, link.read).map_err(Into::into)
}

pub fn get_all_links() -> Result<HashSet<Link>> {
    parse_file(&get_tmp_file()?)
}
