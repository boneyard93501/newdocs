#![allow(non_snake_case)]
use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn hello_fluence() -> String {
    format!("Hello, Fluence!")
}
