use crate::routes::Link;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn get_tmp_file() -> Result<File> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("/tmp/foo.txt")
        .map_err(Into::into)
}

pub fn save_link(link: &Link) -> Result<()> {
    writeln!(get_file_for_write()?, "{},{},{}", link.id, link.url, link.read).map_err(Into::into)
}

fn get_file_for_write() -> Result<File> {
    let file: File = get_tmp_file()?;

    file.set_len(0);
    Ok(file)
}

pub fn get_all_links() -> Result<HashMap<String,(String, bool)>> {
    parse_file(get_tmp_file()?)
}

fn parse_file(file: File) -> Result<HashMap<String, (String, bool)>> {
    let mut link_map: HashMap<String, (String, bool)> = HashMap::new();
    let file = BufReader::new(file);
    for line in file.lines() {
        let line = line?;

        let split = line.split(',').collect::<Vec<&str>>();

        let id: String = split[0].to_string();
        let url: String = split[1].to_string();
        let read: bool = split[2].parse::<bool>()?;

        link_map.insert(id.clone(), (url.clone(), read));
    }
    Ok(link_map)
}
