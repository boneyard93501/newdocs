#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

fn get_connection() {}

fn create_table() {}

fn delete_table() {}

fn add_kv(key: String, value: String) {}

fn update_kv(key: String, value: String) {}

fn get_kv(key: String) {}
