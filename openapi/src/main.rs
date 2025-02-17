use futures_util::StreamExt;
use openapi::{add_functions_from_file, generate_structs_from_file_v2};
use reqwest::{Client, Error, Method};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
generate_structs_from_file_v2!("openapi.json");

// Define a custom error type
struct ApiClient {
    pub host: String,
}
#[add_functions_from_file("openapi.json")]
impl ApiClient {
    fn get_host(&self) -> &str {
        &self.host // Return a reference to the value
    }
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let apiclient = ApiClient {
        host: "https://us-south.ml.cloud.ibm.com".to_string(),
    };

    let request = TextGenRequest {
        project_id: "3856730a-9ffa-4d52-9d0e-395e83789b27".to_string(),
        model_id: "ibm/granite-20b-multilingual".to_string(),
        input: "Your job is to response to the following question: Who was Ceasar?".to_string(),
        parameters: None,
    };

    let apikey = env::var("APIKEY").unwrap();

    let token = apiclient.get_token(apikey.to_string()).await;
    let response = match token {
        Ok(t) => {
            apiclient
                .post_ml_v1_text_generation(t.to_string(), "2023-05-29".to_string(), request)
                .await?
        }
        Err(e) => panic!("There was an error getting the token"),
    };

    println!("{:?}", response);

    Ok(())
}
