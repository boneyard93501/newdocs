use marine_rs_sdk::{marine, MountedBinaryResult};

fn main() {}

#[marine]
pub fn ipfs_request(cmd: Vec<String>) -> MountedBinaryResult {
    ipfs(cmd)
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
