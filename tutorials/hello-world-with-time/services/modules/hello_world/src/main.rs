use chrono;
use marine_rs_sdk::{marine, MountedBinaryResult};
use serde_json;

pub fn main() {}

const WTAPI: &str = "http://worldtimeapi.org/api/timezone/";

fn get_timezone(name: String) -> MountedBinaryResult {
    let url = format!("{}{}", WTAPI, name);
    let curl_cmd = vec![
        "-H".to_string(),
        "Accept: application/json".to_string(),
        "-H".to_string(),
        "Content-Type: application/json".to_string(),
        url,
    ];
    let curl_response = curl_request(curl_cmd);
    curl_response
}

#[marine]
pub fn hello_world(name: String) -> String {
    let response = get_timezone(name.clone());

    if response.error.len() > 0 {
        return response.error;
    }

    let response = String::from_utf8(response.stdout).unwrap();
    if response.contains("error") {
        return response;
    }
    let response: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&response);
    match response {
        Ok(r) => {
            let rfc_dt = r["datetime"].as_str().unwrap();
            let datetime = chrono::DateTime::parse_from_rfc3339(rfc_dt).unwrap();
            let city: Vec<&str> = name.split("/").collect();
            format!("Hello, {} it's {}", city[1], datetime.time().to_string())
        }
        Err(e) => format!("error: {}", e),
    }
}

#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C" {
    pub fn curl_request(cmd: Vec<String>) -> MountedBinaryResult;
}

/*

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(
        config_path = "../../../.fluence/tmp/Config.toml",
        modules_dir = "../../services/modules/hello_world/target/wasm32-wasi/release"
    )]
    fn test_chain(drand: marine_test_env::drand::ModuleInterface) {
        let c_obj = drand.chains(URL.to_string());
        assert_eq!(c_obj.stderr.len(), 0);
        assert!(c_obj.chains.len() > 0);
    }
}
*/
