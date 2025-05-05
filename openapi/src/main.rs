use futures_util::StreamExt;
use openapi::{add_functions_from_file, generate_structs_from_file_v2};
use reqwest::{Client, Error, Method};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

use std::fs;

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

async fn read() {
    let file_path = "duplicates.csv";
    let content = fs::read_to_string(file_path)
        .map(move |content| {
            let unique_lines: std::collections::HashSet<String> =
                content.lines().map(String::from).collect();
            let v: Vec<String> = unique_lines.into_iter().collect();
            v
        })
        .expect("Error reading data from file");
}

async fn gen_response(content: String) -> Result<String, ApiError> {
    let apiclient = ApiClient {
        host: "https://us-south.ml.cloud.ibm.com".to_string(),
    };

    let params = parameters {
        min_new_tokens: Some(0),
        stop_sequences: Some(Vec::new()),
        include_stop_sequence: None,
        repetition_penalty: Some(1.0),
        top_p: None,
        random_seed: None,
        top_k: None,
        decoding_method: Some("greedy".to_string()),
        time_limit: None,
        temperature: None,
        max_new_tokens: Some(500),
    };
    let apikey = env::var("APIKEY").unwrap();
    let prompt = "I have a list of material descriptions in different languages. Examine the list and identify which material descriptions are repeated in more than one language. For example the material PIERRE (FRENCH) is the same as STONE (English).  Return the duplicates and their corresponding language in an ordered list. ";
    let token = apiclient.get_token(apikey.to_string()).await;
    let request = TextGenRequest {
        project_id: "3856730a-9ffa-4d52-9d0e-395e83789b27".to_string(),
        model_id: "meta-llama/llama-3-1-70b-instruct".to_string(),
        input: "".to_string(),
        parameters: Some(params),
    };

    let input = format!("{}  {}\nOutput:", prompt, content);
    let mut r = request.clone();

    r.input = input;

    let response = match token {
        Ok(ref t) => {
            apiclient
                .post_ml_v1_text_generation(t.to_string(), "2023-05-29".to_string(), r)
                .await?
        }
        Err(e) => panic!("There was an error getting the token"),
    };

    if let Some(generated_text) = response
        .get("results")
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(|obj| obj.get("generated_text"))
        .and_then(Value::as_str)
    {
        Ok(generated_text.to_string())
    } else {
        panic!("Error generating!");
    }
}

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    let index_html = include_str!("../index.html");
    HttpResponse::Ok().body(index_html)
}

async fn generate(body: web::Bytes) -> impl Responder {
    let body_string = std::str::from_utf8(&body).unwrap_or("Invalid UTF-8");
    let generation = gen_response(body_string.to_string());
    match generation.await {
        Ok(content) => {
            let message = format!("{}", content.to_string());
            HttpResponse::Ok().body(message)
        }
        Err(e) => {
            let message = "Could not generate results";
            HttpResponse::Ok().body(message)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/generate").route(web::post().to(generate)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
