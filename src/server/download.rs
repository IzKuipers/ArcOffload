use core::time;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    thread::sleep,
};

use crate::types::{connection::ServerConnection, tree::TreeEntry};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use bytes::Bytes;
use colored::Colorize;
use reqwest::blocking::Client;

pub fn download_arcfs_file(
    connection: ServerConnection,
    token: String,
    scoped_path: String,
) -> Bytes {
    let path = URL_SAFE.encode(scoped_path);
    let url = format!(
        "{}/fs/file/get?ac={}&path={}",
        connection.url.clone(),
        connection.authcode.clone(),
        path
    );

    let call = Client::new().get(url).bearer_auth(token).send();

    match call {
        Ok(data) => {
            let text = data.bytes().unwrap();

            return text;
        }
        Err(_) => Bytes::new(),
    }
}

pub fn write_file(path: String, bytes: Bytes, destination: String) -> bool {
    let mut dir_path = PathBuf::new();

    dir_path.push(&destination);
    dir_path.push(&path);
    dir_path.pop();

    let mut file_path = PathBuf::new();

    file_path.push(&destination);
    file_path.push(&path);

    create_dir_all(dir_path.as_os_str().to_str().unwrap()).unwrap();

    let mut file = File::create(file_path.as_os_str().to_str().unwrap()).unwrap();

    let result = file.write_all(&bytes);

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn write_full_tree(
    tree: Vec<TreeEntry>,
    token: String,
    connection: ServerConnection,
    dest: String,
) {
    let length = tree.len();

    for i in 0..length {
        let item = tree.get(i).unwrap();
        let scoped_path = item.scoped_path.clone();

        println!(
            "[{}/{}] {}: {}",
            i + 1,
            length,
            "Writing".blue().bold(),
            scoped_path.clone()
        );

        let bytes = download_arcfs_file(connection.clone(), token.clone(), scoped_path.clone());

        write_file(scoped_path.clone(), bytes, dest.clone());

        let dur = time::Duration::from_millis(55);

        sleep(dur);
    }
}
