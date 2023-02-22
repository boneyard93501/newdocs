use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub struct IOResult {
    pub stdout: String,
    pub stderr: String,
}

#[marine]
pub fn read_file(name: String) -> IOResult {
    let response: Vec<u8> = file_get(name);
    if response == b"error while reading file".to_vec() {
        return IOResult {
            stdout: "".to_string(),
            stderr: "error reading file".to_string(),
        };
    } else {
        IOResult {
            stdout: String::from_utf8_lossy(&response).to_string(),
            stderr: "".to_string(),
        }
    }
}

#[marine]
pub fn write_file(name: String, payload: String) -> IOResult {
    let content = payload.as_bytes().to_vec();
    let response = file_put(name, content);

    let result = response.as_str().contains("file can't be written:");
    if result {
        return IOResult {
            stdout: "".to_string(),
            stderr: response,
        };
    } else {
        IOResult {
            stdout: "OK".to_string(),
            stderr: "".to_string(),
        }
    }
}

#[marine]
pub fn rm_file(name: String) -> IOResult {
    let result = file_remove(name);
    if &result == "OK" {
        return IOResult {
            stdout: result,
            stderr: "".to_string(),
        };
    } else {
        IOResult {
            stdout: "".to_string(),
            stderr: result,
        }
    }
}

#[marine]
#[link(wasm_import_module = "filesys_adapter")] // 1
extern "C" {
    #[link_name = "get"]
    pub fn file_get(file_name: String) -> Vec<u8>; // 2

    #[link_name = "put"]
    pub fn file_put(name: String, file_content: Vec<u8>) -> String;

    #[link_name = "rm"]
    pub fn file_remove(name: String) -> String;
}
