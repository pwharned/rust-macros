use openapi::{add_functions_from_file, generate_structs_from_file_v2};
use reqwest::{Client, Error, Method};
use serde::{Deserialize, Serialize};

generate_structs_from_file_v2!("openapi.json");

struct ApiClient {
    host: String,
}
#[add_functions_from_file("openapi.json")]
impl ApiClient {
    fn get_host(&self) -> &str {
        &self.host // Return a reference to the value
    }
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let apiclient = ApiClient {
        host: "http://localhost:8080".to_string(),
    };

    Ok(())
}
