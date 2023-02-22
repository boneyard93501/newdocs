// #![allow(non_snake_case)]//
/// You can read or write files from the file system if there is permission to
/// use the directory(ies) described in the deployment configuration.
use marine_rs_sdk::marine;

use std::fs;
use std::path::PathBuf;

const SITES_DIR: &str = "/sites/";

pub fn main() {}

#[marine]
pub fn put(name: String, file_content: Vec<u8>) -> String {
    let rpc_tmp_filepath = format!("{}{}", SITES_DIR, name);

    let result = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = result {
        return format!("file can't be written: {}", e);
    }

    String::from("Ok")
}

#[marine]
pub fn get(file_name: String) -> Vec<u8> {
    let tmp_filepath = format!("{}{}", SITES_DIR, file_name);
    fs::read(tmp_filepath).unwrap_or_else(|_| b"error while reading file".to_vec())
}

#[marine]
pub fn rm(name: String) -> String {
    let rpc_tmp_filepath = format!("{}{}", SITES_DIR, name);
    let result = fs::remove_file(rpc_tmp_filepath);
    match result {
        Ok(_) => "OK".to_string(),
        Err(e) => e.to_string(),
    }
}
