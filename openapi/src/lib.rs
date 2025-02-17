extern crate proc_macro;
use parse;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;
use syn::LitStr;
use syn::Type;
use syn::{parse_macro_input, ItemImpl};
#[derive(Debug)]
enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Float(f64),
}

impl Value {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Value::Int(val) => quote! { #val },
            Value::Str(val) => quote! { #val },
            Value::Bool(val) => quote! { #val },
            Value::Float(val) => quote! { #val },
        }
    }
}

fn remove_values_inside_brackets(input: &str) -> String {
    let re = Regex::new(r"\{[^}]+\}").unwrap();
    let result = re.replace_all(input, "{}");
    "{}".to_string() + &result
}

fn extract_parts_helper(route: &str) -> (Vec<String>, Vec<String>) {
    let mut parts_not_in_brackets = Vec::new();
    let mut parts_in_brackets = Vec::new();
    let mut current_part = String::new();
    let mut in_brackets = false;

    for c in route.chars() {
        if c == '{' {
            if !current_part.is_empty() {
                current_part.push('{');
                current_part.push('}');

                parts_not_in_brackets.push(current_part.clone());
                current_part.clear();
            }
            in_brackets = true;
        } else if c == '}' {
            parts_in_brackets.push(current_part.clone());
            current_part.clear();
            in_brackets = false;
        } else {
            current_part.push(c);
        }
    }

    if !current_part.is_empty() {
        parts_not_in_brackets.push(current_part);
    }

    (parts_not_in_brackets, parts_in_brackets)
}

#[proc_macro_attribute]
pub fn add_functions_from_file(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute input for the file path
    let relative_path = parse_macro_input!(attr as syn::LitStr).value();

    // Construct the absolute path
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let file_path = PathBuf::from(manifest_dir).join(relative_path);

    // Read the JSON file
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");

    let openapi: OpenApiSpec =
        serde_json::from_str(&file_content).expect("Json was not well formatted");

    // Extract the name of the struct
    let input = parse_macro_input!(item as ItemImpl);
    let mut output = quote! { #input };
    output.extend(quote! {
    use serde_json::Value;
    #[derive(Debug)]
    enum ApiError {
        Reqwest(reqwest::Error),
        SerdeJson(serde_json::Error),
    }

    // Implement the Display trait for MyError
    impl fmt::Display for ApiError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ApiError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
                ApiError::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
            }
        }
    }

    // Implement the From trait for MyError to convert from ReqwestError
    impl From<reqwest::Error> for ApiError {
        fn from(error: reqwest::Error) -> Self {
            ApiError::Reqwest(error)
        }
    }

    // Implement the From trait for MyError to convert from SerdeJsonError
    impl From<serde_json::Error> for ApiError {
        fn from(error: serde_json::Error) -> Self {
            ApiError::SerdeJson(error)
        }
    }


        });
    let struct_name = match *input.self_ty {
        Type::Path(ref type_path) => {
            if let Some(segment) = type_path.path.segments.first() {
                segment.ident.clone()
            } else {
                panic!("Expected a path segment");
            }
        }
        _ => panic!("Expected a type path"),
    };

    let security = openapi.components.unwrap().security_schemes.unwrap();
    let token_url = security.oauth2.flows.x_apikey.token_url;
    let grant_type = security.oauth2.flows.x_apikey.grant_type;
    let secret_type = security.oauth2.flows.x_apikey.secret_keys.first().unwrap();

    let secret_syn = syn::Ident::new(secret_type, proc_macro2::Span::call_site());

    let new_function = quote! {
                    impl #struct_name {
    async fn get_token (&self, #secret_syn: String ) -> Result<String, ApiError> {


                let client = Client::new();
                let body = format!("grant_type={}&{}={}",#grant_type,#secret_type,#secret_syn);

                let response = client.post(#token_url).header("Content-Type", "application/x-www-form-urlencoded").body(body).send().await?;
                let json: Value = response.json().await?;

            Ok(json.get("access_token")
            .and_then(Value::as_str)
            .map(String::from).unwrap())

        }

                    }
                };
    output.extend(new_function);

    for path_item in openapi.paths.paths {
        for item in path_item.1.methods {
            let method_name: &str = item.0.as_ref();
            let func_name: String = format!(
                "{}{}",
                method_name,
                path_item
                    .0
                    .replace('/', "_")
                    .replace('{', "by_")
                    .replace('}', ""),
            );
            let response = item
                .1
                .clone()
                .unwrap()
                .responses
                .responses
                .get("200")
                .unwrap()
                .content
                .clone()
                .unwrap();

            let request = item.1.clone().and_then(move |operation| {
                operation.clone().request_body.map(|request| {
                    let content = request.content;
                    let request_keys: Vec<_> = content.keys().cloned().collect();
                    let first_request_key = request_keys.first().unwrap().clone();

                    match (first_request_key.as_ref(), content.get(&first_request_key)) {
                        ("application/json", Some(value)) => {
                            if let Some(ref_path) = &value.schema.ref_ {
                                let parts: Vec<&str> = ref_path.split('/').collect();
                                if let Some(last) = parts.last() {
                                    let arg_name = syn::Ident::new(
                                        &last.clone().to_lowercase(),
                                        proc_macro2::Span::call_site(),
                                    );
                                    let arg_type =
                                        syn::Ident::new(last, proc_macro2::Span::call_site());
                                    (
                                        arg_name.clone(),
                                        quote! {
                                            #arg_name: #arg_type,
                                        },
                                    )

                                    // Now you can safely use last
                                } else {
                                    let arg_type =
                                        syn::Ident::new("String", proc_macro2::Span::call_site());
                                    let arg_name =
                                        syn::Ident::new("string", proc_macro2::Span::call_site());
                                    (
                                        arg_name.clone(),
                                        quote! {
                                            #arg_name: #arg_type,
                                        },
                                    )
                                }
                            } else {
                                let arg_type =
                                    syn::Ident::new("String", proc_macro2::Span::call_site());
                                let arg_name =
                                    syn::Ident::new("string", proc_macro2::Span::call_site());
                                (
                                    arg_name.clone(),
                                    quote! {
                                        #arg_name: #arg_type,
                                    },
                                )
                            }
                        }

                        (&_, _) => todo!(),
                    }
                })
            });

            let keys: Vec<_> = response.keys().cloned().collect();
            let first_key = keys.first().unwrap().clone();

            let resp = match (first_key.as_ref(), response.get(&first_key)) {
                ("application/json", Some(value)) => {
                    if let Some(ref_path) = &value.schema.ref_ {
                        let parts: Vec<&str> = ref_path.split('/').collect();
                        if let Some(last) = parts.last() {
                            let response_type =
                                syn::Ident::new(last, proc_macro2::Span::call_site());

                            // Now you can safely use last
                            (
                                "Value",
                                quote! {

                                                                let mut bytes = response.bytes().await.unwrap();



                                                                match  serde_json::from_slice::<Value>(&bytes)
                                {
                                                                    Ok(data) => Ok(data),
                                                                    Err(e) => Err(ApiError::from(e ))
                                                                    }



                                                                                },
                            )
                        } else {
                            (
                                "String",
                                quote! {
                                    match response.text().await {
                                    Ok(data) => Ok(data),
                                    Err(e) => Err(ApiError::from(e))

                                    }
                                },
                            )
                        }
                    } else {
                        (
                            "String",
                            quote! {

                            match response.text().await {
                            Ok(data) => Ok(data),
                            Err(e) => Err(ApiError::from(e))

                                                               }


                                                       },
                        )
                    }
                }

                ("text/event-stream", Some(value)) => (
                    "String",
                    quote! {

                                                                       let mut bytes = Vec::new();
                                                                                       let mut stream = response.bytes_stream();

                                while let Some(event) = stream.next().await {
                                    match event {
                                        Ok(event_bytes) => bytes.extend_from_slice(&event_bytes),
                                        Err(e) => return Err(ApiError::Reqwest(e)),
                                    }
                                }

                        let strings: Vec<String> = bytes.split(|&byte| byte == 0)
                    .filter(|slice| !slice.is_empty())
                    .map(|slice| {
                        let vec_u8 = slice.to_vec();
                        String::from_utf8_lossy(&vec_u8).to_string()
                    })
                    .collect();
                                                                   let data = strings.join("");
                        Ok(data)

                                                                               },
                ),
                (&_, _) => todo!(),
            };

            let response_type = resp.0;
            let response_handling = resp.1;

            let (parts_not_in_brackets, parts_in_brackets) = extract_parts(&path_item.0);
            let arg_names: Vec<syn::Ident> = parts_in_brackets
                .iter()
                .map(|arg| syn::Ident::new(arg, proc_macro2::Span::call_site()))
                .collect();

            let arg_types: Vec<syn::Type> = parts_in_brackets
                .iter()
                .map(|_| syn::parse_str::<syn::Type>("&String").unwrap())
                .collect();

            let params = item.1.clone().and_then(move |o| o.parameters);
            let args_iter = arg_names.iter().zip(arg_types.iter());
            let mut func_args: Vec<proc_macro2::TokenStream> = args_iter
                .map(|(name, ty)| {
                    quote! { #name: #ty }
                })
                .collect();

            let mut request_parameters = Vec::new();
            func_args.push(quote! {token:String});
            let mut headers = Vec::new();
            headers.push(quote! {.header("Authorization", format!("Bearer: {}",token)) });

            if let Some(parameters) = params {
                for p in parameters {
                    let pname = &p.name;
                    let name = syn::Ident::new(&p.name, proc_macro2::Span::call_site());
                    let j_type = p.schema.unwrap().type_.unwrap();
                    let ty: syn::Ident = match j_type.as_str() {
                        "string" => syn::parse_str("String").expect("invalid type"),
                        _ => syn::parse_str("String").expect("invalid type"),
                    };
                    func_args.push(quote! {#name: #ty});
                    request_parameters.push(quote! {
                        (#pname, #name)
                    });
                }
            }
            if let Some(ref value) = request {
                func_args.push(value.1.clone())
            };

            let (parts_not_in_brackets1, parts_in_brackets1) = extract_parts_helper(&path_item.0);

            let impl_name = syn::Ident::new(&func_name, proc_macro2::Span::call_site());
            let meth_name = syn::Ident::new(method_name, proc_macro2::Span::call_site());

            let blank_url = syn::parse::<LitStr>(
                remove_values_inside_brackets(&path_item.0)
                    .to_token_stream()
                    .into(),
            )
            .unwrap()
            .value();

            let request_implmentation = match method_name.to_uppercase().as_str() {
                "GET" => quote! {

                let response = client.get(url).send().await?
                },
                "PATCH" => quote! {
                 let response =       client.patch(url).send().await?
                },
                "POST" => {
                    let request_body_name = request.unwrap().0;
                    quote! {
                    let params =  [#(#request_parameters),*];
                    let response = client.post(url)#(#headers),* .json(&#request_body_name).query(&params).send().await?}
                }
                "PUT" => quote! {
                let response = client.put(url).send().await?},
                &_ => todo!("{}", method_name),
            };

            let mut new_function = proc_macro2::TokenStream::new();
            let response_type_syn = syn::Ident::new(response_type, proc_macro2::Span::call_site());
            if arg_names.is_empty() {
                new_function = quote! {
                                impl #struct_name {
                async fn #impl_name (&self, #(#func_args),*) -> Result<#response_type_syn, ApiError> {

                            let url = format!(#blank_url, self.get_host());
                            let client = Client::new();

                            #request_implmentation;



                            #response_handling
                    }

                                }
                            };
            } else {
                new_function = quote! {
                                           impl #struct_name {
                           async fn #impl_name (&self, #(#func_args),*) -> Result<response_type_syn, ApiError> {

                                           let method_name =stringify!(#meth_name);
                                   let base_url = format!(#blank_url,self.get_host(), #(#arg_names),* );


                                       let method: Method = Method::from_bytes(method_name.as_bytes() ).unwrap();
                                       let client = Client::new();



                                       let req = client.request(method, self.get_host());
                                   let response = req
                                       .send()
                                       .await?;

                match response.json().await {
                                       Ok(data) => Ok(data),
                                       Err(e) => Err(ApiError::from(e))

                                                                          }

                               }

                                           }
                                       };
            }

            output.extend(new_function);
        }
    }

    let testoutput = quote! {};
    println!("{}", output);

    write_output_to_file(&output, &testoutput);
    TokenStream::from(output)
}

fn extract_parts(path: &str) -> (String, Vec<String>) {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut parts_in_brackets = Vec::new();
    let mut parts_not_in_brackets = String::new();

    let mut last_end = 0;

    for cap in re.captures_iter(path) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();

        // Append the part not in brackets
        parts_not_in_brackets.push_str(&path[last_end..start]);

        // Capture the part in brackets
        parts_in_brackets.push(cap.get(1).unwrap().as_str().to_string());

        last_end = end;
    }

    // Append the remaining part not in brackets
    parts_not_in_brackets.push_str(&path[last_end..]);

    (parts_not_in_brackets, parts_in_brackets)
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenApiSpec {
    openapi: String,
    info: Info,
    servers: Option<Vec<Server>>,
    paths: Paths,
    components: Option<Components>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    title: String,
    description: Option<String>,
    version: String,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    url: String,
    description: Option<String>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Paths {
    #[serde(flatten)]
    paths: std::collections::HashMap<String, PathItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PathItem {
    #[serde(flatten)]
    methods: std::collections::HashMap<String, Option<Operation>>,
    //post: Option<Operation>,
    //put: Option<Operation>,
    //delete: Option<Operation>,
    // Add other HTTP methods as needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Operation {
    tags: Option<Vec<String>>,
    summary: Option<String>,
    description: Option<String>,
    operation_id: Option<String>,
    parameters: Option<Vec<Parameter>>,
    #[serde(rename = "requestBody")]
    request_body: Option<RequestBody>,
    responses: Responses,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Parameter {
    name: String,
    #[serde(rename = "in")]
    in_: String,
    description: Option<String>,
    required: Option<bool>,
    schema: Option<Schema>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RequestBody {
    content: std::collections::HashMap<String, MediaType>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaType {
    schema: Schema,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Responses {
    #[serde(flatten)]
    responses: std::collections::HashMap<String, Response>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Response {
    description: String,
    content: Option<std::collections::HashMap<String, MediaType>>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Components {
    schemas: Option<std::collections::HashMap<String, Schema>>,
    #[serde(rename = "securitySchemes")]
    security_schemes: Option<SecuritySchemes>, // Add other fields as needed

                                               // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Schema {
    #[serde(rename = "type")]
    type_: Option<String>,
    properties: Option<std::collections::HashMap<String, Property>>,
    #[serde(rename = "$ref")]
    ref_: Option<String>,
    required: Option<Vec<String>>,
    // Add other fields as needed
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Property {
    #[serde(rename = "type")]
    type_: Option<String>,
    format: Option<String>,
    properties: Option<std::collections::HashMap<String, Property>>,
    items: Option<Box<Property>>,
    // Add other fields as needed
}

#[proc_macro]
pub fn register_handlers(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as LitStr);
    let file_path = input.value();
    let mut file = std::fs::File::open(&file_path).expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read file");
    let input: syn::File = syn::parse_str(&content).expect("Unable to parse file content");
    let mut handlers = vec![];
    for item in input.items {
        if let syn::Item::Fn(func) = item {
            let fn_name = func.sig.ident.to_string();
            if fn_name.ends_with("_handler") {
                let fn_ident = syn::Ident::new(&fn_name, proc_macro2::Span::call_site());
                handlers.push(fn_ident);
            }
        }
    }

    let expanded = quote! { pub fn register_all_handlers(cfg: &mut actix_web::web::ServiceConfig) { #(cfg.service(#handlers);)* } };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn generate_structs_from_file_v2(attr: TokenStream) -> TokenStream {
    // Parse the attribute input for the file path
    let relative_path = parse_macro_input!(attr as LitStr).value();

    // Construct the absolute path
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let file_path = PathBuf::from(manifest_dir).join(relative_path);

    // Read the JSON file
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");

    let openapi: OpenApiSpec =
        serde_json::from_str(&file_content).expect("Json was not well formatted");

    // Generate structs based on the JSON data
    let mut output = quote! {};

    if let Some(components) = openapi.components {
        if let Some(schemas) = components.schemas {
            for (struct_name, struct_def) in schemas {
                let struct_name = syn::Ident::new(&struct_name, proc_macro2::Span::call_site());
                let fields = generate_fields(&struct_def, &mut output);

                let new_struct = quote! {
                    #[derive(Deserialize, Debug,Serialize)]
                    pub struct #struct_name {
                        #(#fields)*
                    }
                };

                output.extend(new_struct);
            }
        }
    }
    println!("{}", output);
    output.into()
}

fn generate_fields(
    schema: &Schema,
    output: &mut proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    let mut fields = Vec::new();
    let mut req = Vec::new();
    if let Some(required) = &schema.required {
        for r in required {
            req.push(r.clone());
        }
    }

    if let Some(properties) = &schema.properties {
        for (name, field) in properties {
            let field_name = syn::Ident::new(name, proc_macro2::Span::call_site());

            let field_type_str = field.type_.as_ref().unwrap();
            let is_required = req.contains(&name);

            let field_ty: syn::Type = match field_type_str.as_str() {
                "string" => match is_required {
                    true => syn::parse_str("String").expect("Invalid type"),
                    false => syn::parse_str("Option<String>").expect("Invalid type"),
                },
                "integer" => match is_required {
                    true => syn::parse_str("i32").expect("Invalid type"),
                    false => syn::parse_str("Option<i32>").expect("Invalid type"),
                },

                "number" => match is_required {
                    true => syn::parse_str("f32").expect("Invalid type"),
                    false => syn::parse_str("Option<f32>").expect("Invalid type"),
                },

                "boolean" => match is_required {
                    true => syn::parse_str("bool").expect("Invalid type"),
                    false => syn::parse_str("Option<bool>").expect("Invalid type"),
                },

                "object" | "array" => {
                    // Recursively generate the nested struct
                    let nested_struct_name =
                        syn::Ident::new(&format!("{}", field_name), proc_macro2::Span::call_site());
                    let nested_fields = generate_fields_from_properties(&field.properties, output);

                    let nested_struct = quote! {
                        #[derive(Deserialize, Debug,Serialize)]
                        pub struct #nested_struct_name {
                            #(#nested_fields)*
                        }
                    };

                    output.extend(nested_struct);

                    match is_required {
                        true => syn::parse_str(&format!("{}", nested_struct_name))
                            .expect("Invalid type"),
                        false => syn::parse_str(&format!("Option<{}>", nested_struct_name))
                            .expect("Invalid type"),
                    }
                }
                _ => panic!(
                    "{}",
                    format!("Unsupported type {}", field_type_str.as_str())
                ),
            };

            let field_def = quote! {
                pub #field_name: #field_ty,
            };

            fields.push(field_def);
        }
    }

    fields
}

fn generate_fields_from_properties(
    properties: &Option<std::collections::HashMap<String, Property>>,
    output: &mut proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    let mut fields = Vec::new();
    let is_required = true;
    if let Some(properties) = &properties {
        for (field_name, field) in properties {
            let field_name = syn::Ident::new(field_name, proc_macro2::Span::call_site());

            let field_type_str = field.type_.as_ref().unwrap();
            let field_ty: syn::Type = match field_type_str.as_str() {
                "string" => match is_required {
                    true => syn::parse_str("String").expect("Invalid type"),
                    false => syn::parse_str("Option<String>").expect("Invalid type"),
                },
                "integer" => match is_required {
                    true => syn::parse_str("i32").expect("Invalid type"),
                    false => syn::parse_str("Option<i32>").expect("Invalid type"),
                },

                "number" => match is_required {
                    true => syn::parse_str("f32").expect("Invalid type"),
                    false => syn::parse_str("Option<f32>").expect("Invalid type"),
                },

                "boolean" => match is_required {
                    true => syn::parse_str("bool").expect("Invalid type"),
                    false => syn::parse_str("Option<bool>").expect("Invalid type"),
                },

                "object" | "array" => {
                    // Recursively generate the nested struct
                    let nested_struct_name =
                        syn::Ident::new(&format!("{}", field_name), proc_macro2::Span::call_site());
                    let nested_fields = generate_fields_from_properties(&field.properties, output);

                    let nested_struct = quote! {
                        #[derive(Deserialize, Debug,Serialize)]
                        pub struct #nested_struct_name {
                            #(#nested_fields)*
                        }
                    };

                    output.extend(nested_struct);

                    syn::parse_str(&format!("{}", nested_struct_name)).expect("Invalid type")
                }
                _ => panic!(
                    "{}",
                    format!("Unsupported type {}", field_type_str.as_str())
                ),
            };

            let field_def = quote! {
                pub #field_name: #field_ty,
            };

            fields.push(field_def);
        }
    }
    fields
}

fn write_output_to_file(output: &proc_macro2::TokenStream, testoutput: &proc_macro2::TokenStream) {
    let dest_path = Path::new("generated_handlers.rs");
    let mut file = File::create(dest_path).unwrap();
    file.write_all(output.to_string().as_bytes()).unwrap();
    let outtestoutput = quote! {
        mod tests {
            use super::*;
            use std::env;
            use actix_web::{App, test, web, http::StatusCode};

            #testoutput
        }
    };

    file.write_all(outtestoutput.to_string().as_bytes())
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct SecuritySchemes {
    oauth2: Oauth2,
}

#[derive(Serialize, Deserialize, Debug)]
struct Oauth2 {
    #[serde(rename = "type")]
    type_field: String,
    flows: Flows,
}

#[derive(Serialize, Deserialize, Debug)]
struct Flows {
    #[serde(rename = "x-apikey")]
    x_apikey: XApikey,
}

#[derive(Serialize, Deserialize, Debug)]
struct XApikey {
    #[serde(rename = "tokenUrl")]
    token_url: String,
    #[serde(rename = "grantType")]
    grant_type: String,
    #[serde(rename = "secretKeys")]
    secret_keys: Vec<String>,
    #[serde(rename = "paramKeys")]
    param_keys: Vec<String>,
    scopes: std::collections::HashMap<String, String>,
}
