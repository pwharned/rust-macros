extern crate proc_macro;
use parse;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use regex::Regex;
use serde::{Deserialize, Serialize};
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
    for path_item in openapi.paths.paths {
        println!("{}", path_item.0);

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
            let (parts_not_in_brackets, parts_in_brackets) = extract_parts(&path_item.0);
            let arg_names: Vec<syn::Ident> = parts_in_brackets
                .iter()
                .map(|arg| syn::Ident::new(arg, proc_macro2::Span::call_site()))
                .collect();

            let arg_types: Vec<syn::Type> = parts_in_brackets
                .iter()
                .map(|_| syn::parse_str::<syn::Type>("&String").unwrap())
                .collect();

            let args_iter = arg_names.iter().zip(arg_types.iter());
            let func_args: Vec<proc_macro2::TokenStream> = args_iter
                .map(|(name, ty)| {
                    quote! { #name: #ty }
                })
                .collect();

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
            let mut new_function = proc_macro2::TokenStream::new();
            if arg_names.is_empty() {
                new_function = quote! {
                                impl #struct_name {
                async fn #impl_name (&self, #(#func_args),*) -> Result<Vec<#struct_name>, reqwest::Error> {

                                let func_name = stringify!(#impl_name);
                                let method_name =stringify!(#meth_name);

                        let base_url = self.get_host();
                            let url = format!(#blank_url, self.get_host());
                            let method: Method = Method::from_bytes(method_name.as_bytes() ).unwrap();
                            let client = Client::new();



                        let response = match method_name {
                            "GET" => client.get(url).send().await?,
                            "PATCH" => client.patch(url).send().await?,
                            "POST" => client.post(url).send().await?,
                            "PUT" => client.put(url).send().await?,

                                _ => reqwest::get(url).await?
                        };

                        let data = response.json::<Vec<#struct_name>>().await?;
                        Ok(data)
                    }

                                }
                            };
            } else {
                new_function = quote! {
                                impl #struct_name {
                async fn #impl_name (&self, #(#func_args),*) -> Result<Vec<User>, reqwest::Error> {

                                let func_name = stringify!(#impl_name);
                                let method_name =stringify!(#meth_name);
                        let base_url = format!(#blank_url,self.get_host(), #(#arg_names),* );
                            //let test_url = format!("{}", #(#arg_names),* );


                            let method: Method = Method::from_bytes(method_name.as_bytes() ).unwrap();
                            let client = Client::new();



                            let req = client.request(method, self.get_host());
                        let response = req
                            .send()
                            .await?;

                        let data = response.json::<Vec<User>>().await?;
                        Ok(data)
                    }

                                }
                            };
            }

            output.extend(new_function);
        }
    }

    println!("{}", output);
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
    // Add other fields as needed
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

#[derive(Serialize, Deserialize, Debug)]
struct Operation {
    tags: Option<Vec<String>>,
    summary: Option<String>,
    description: Option<String>,
    operation_id: Option<String>,
    parameters: Option<Vec<Parameter>>,
    request_body: Option<RequestBody>,
    responses: Responses,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Parameter {
    name: String,
    #[serde(rename = "in")]
    in_: String,
    description: Option<String>,
    required: Option<bool>,
    schema: Option<Schema>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    content: std::collections::HashMap<String, MediaType>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaType {
    schema: Schema,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Responses {
    #[serde(flatten)]
    responses: std::collections::HashMap<String, Response>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    description: String,
    content: Option<std::collections::HashMap<String, MediaType>>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Components {
    schemas: Option<std::collections::HashMap<String, Schema>>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Schema {
    #[serde(rename = "type")]
    type_: Option<String>,
    properties: Option<std::collections::HashMap<String, Property>>,
    // Add other fields as needed
}
#[derive(Serialize, Deserialize, Debug)]
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
            println!("{}", fn_name);
            if fn_name.ends_with("_handler") {
                println!("registering handler {}", fn_name);
                let fn_ident = syn::Ident::new(&fn_name, proc_macro2::Span::call_site());
                handlers.push(fn_ident);
            }
        }
    }

    let expanded = quote! { pub fn register_all_handlers(cfg: &mut actix_web::web::ServiceConfig) { #(cfg.service(#handlers);)* } };
    println!("{}", expanded);
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
                    #[derive(Deserialize, Debug)]
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

    if let Some(properties) = &schema.properties {
        for (field_name, field) in properties {
            let field_name = syn::Ident::new(field_name, proc_macro2::Span::call_site());

            let field_type_str = field.type_.as_ref().unwrap();
            let field_ty: syn::Type = match field_type_str.as_str() {
                "string" => syn::parse_str("String").expect("Invalid type"),
                "integer" => syn::parse_str("i32").expect("Invalid type"),
                "number" => syn::parse_str("f32").expect("Invalid type"),
                "boolean" => syn::parse_str("bool").expect("Invalid type"),
                "object" | "array" => {
                    // Recursively generate the nested struct
                    let nested_struct_name =
                        syn::Ident::new(&format!("{}", field_name), proc_macro2::Span::call_site());
                    let nested_fields = generate_fields_from_properties(&field.properties, output);

                    let nested_struct = quote! {
                        #[derive(Deserialize, Debug)]
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

fn generate_fields_from_properties(
    properties: &Option<std::collections::HashMap<String, Property>>,
    output: &mut proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    let mut fields = Vec::new();
    if let Some(properties) = &properties {
        for (field_name, field) in properties {
            let field_name = syn::Ident::new(field_name, proc_macro2::Span::call_site());

            let field_type_str = field.type_.as_ref().unwrap();
            let field_ty: syn::Type = match field_type_str.as_str() {
                "string" => syn::parse_str("String").expect("Invalid type"),
                "integer" => syn::parse_str("i32").expect("Invalid type"),
                "number" => syn::parse_str("f32").expect("Invalid type"),
                "boolean" => syn::parse_str("bool").expect("Invalid type"),
                "object" | "array" => {
                    // Recursively generate the nested struct
                    let nested_struct_name =
                        syn::Ident::new(&format!("{}", field_name), proc_macro2::Span::call_site());
                    let nested_fields = generate_fields_from_properties(&field.properties, output);

                    let nested_struct = quote! {
                        #[derive(Deserialize, Debug)]
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
