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
                async fn #impl_name (&self, #(#func_args),*) -> Result<Vec<User>, reqwest::Error> {

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

                        let data = response.json::<Vec<User>>().await?;
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

    //println!("{}", output);
    TokenStream::from(output)
}
#[proc_macro]
pub fn generate_structs_from_ddl(attr: TokenStream) -> TokenStream {
    let relative_path = parse_macro_input!(attr as syn::LitStr).value();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let file_path = PathBuf::from(manifest_dir).join(relative_path);
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");
    let sql = file_content.split(";");
    let mut output = quote! {};
    let mut testoutput = quote! {};
    for statement in sql.into_iter() {
        let content = statement.replace("\n", "");
        let stmt = parse::create_table_parser().parse(&content);

        if let Err(err) = stmt {
            println!("{:?}", err);
            continue;
        }

        let ddl = stmt.unwrap();
        let table_name = &ddl.0.name;
        let struct_name = syn::Ident::new(&ddl.0.name, proc_macro2::Span::call_site());
        let columns = ddl.0.columns;
        let fieldst: Vec<(_, Value, String, proc_macro2::Ident)> = columns
            .iter()
            .map(|col| {
                let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());

                let field_type_str = col.dtype.to_string();
                match field_type_str.to_uppercase().as_str() {
                    "VARCHAR" => (
                        syn::parse_str::<syn::Ident>("String").expect("Invalid type"),
                        Value::Str("Test".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "INT" => (
                        syn::parse_str("i32").expect("Invalid type"),
                        Value::Int(1),
                        field_type_str,
                        field_name,
                    ),
                    "INTEGER" => (
                        syn::parse_str("i32").expect("Invalid type"),
                        Value::Int(1),
                        field_type_str,
                        field_name,
                    ),

                    "BOOL" => (
                        syn::parse_str("bool").expect("Invalid type"),
                        Value::Bool(true),
                        field_type_str,
                        field_name,
                    ),
                    "TEXT" => (
                        syn::parse_str("String").expect("Invalid type"),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "TEXT[]" => (
                        syn::parse_str("String").expect("Invalid type"),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "DOUBLE" => (
                        syn::parse_str("f64").expect("Invalid type"),
                        Value::Float(0.0),
                        field_type_str,
                        field_name,
                    ),
                    "UUID" => (
                        syn::parse_str("String").expect("Invalid type"),
                        Value::Str("acde070d-8c4c-4f0d-9d8a-162843c10333".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "CHARACTER" => (
                        syn::parse_str("String").expect("Invalid type"),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "TIMESTAMPTZ" => (
                        syn::parse_str("String").expect("Invalid type"),
                        Value::Str("2016-01-25 10:10:10.555555-05:00".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "TIMESTAMP" => (
                        syn::parse_str("String").expect("Invalid type"),
                        Value::Str("2024-10-16 14:30:00".to_string()),
                        field_type_str,
                        field_name,
                    ),
                    "BOOLEAN" => (
                        syn::parse_str::<syn::Ident>("bool").expect("Invalid type"),
                        Value::Bool(true),
                        field_type_str,
                        field_name,
                    ),

                    _ => (
                        syn::parse_str("")
                            .expect(&format!("Invalid type found {}", field_type_str)),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                    ),
                }
            })
            .collect();

        let fields = fieldst.into_iter().map({
            |(field_ty, _, _, field_name)| {
                quote! {
                    pub #field_name: Option<#field_ty>,
                }
            }
        });
        let fields2 = columns.iter().map(|col| {
            let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());

            quote! {
                   .bind(json.#field_name)
            }
        });
        let fields3 = columns.iter().map(|col| {
            let colname = &col.name;
            let field_name = syn::Ident::new(colname, proc_macro2::Span::call_site());
            quote! {
                            if !self.#field_name.is_none(){
                                fields.push((#colname, &self.#field_name as &dyn std::fmt::Debug) );
                            }
            }
        });
        let cols = columns
            .iter()
            .map(move |col| {
                let binding = col.dtype.to_lowercase();
                let dtype = binding.as_str();
                match dtype {
                    "uuid" => format!("cast({} as varchar) as {}", col.colname, col.colname),
                    "timestamptz" => format!("cast({} as varchar) as {}", col.colname, col.colname),
                    "text[]" => format!("array_to_string({}, ',') as {}", col.colname, col.colname),
                    "timestamp" => {
                        format!("cast({} as varchar) as {}", col.colname, col.colname)
                    }

                    _ => col.colname.clone(),
                }
            })
            .collect::<Vec<_>>()
            .join(",");

        let primary_keys: Vec<&parse::PrimaryKey> = ddl
            .0
            .constraints
            .iter()
            .filter_map(|item| {
                if let parse::Constraint::PrimaryKey(foo) = item {
                    Some(foo)
                } else {
                    None
                }
            })
            .collect();

        let new_struct = quote! {
            #[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
            pub struct #struct_name {
                #(#fields)*
            }

        };
        let new_struct2 = quote! {
                       impl #struct_name {
               pub fn non_null_fields(&self) -> Vec<(&str, &dyn std::fmt::Debug)>{
                           let mut fields = Vec::new();
                           #(#fields3)*
                           fields
                       }
        pub fn bind_fields(&self,  sqlx_query:&mut sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> ) -> (){
                       }
                       }
                   };
        for pkey in primary_keys {
            let pkey_cols = pkey.columns.clone();
            let mut route: String = "/".to_owned() + table_name;
            for c in &pkey_cols {
                route.push_str(&format!("/{{{}}}", c));
            }

            let where_cols = pkey_cols
                .iter()
                .enumerate()
                .map(|(index, item)| format!("{} = ${}", item, index + 1))
                .collect::<Vec<String>>()
                .join(" AND ");
            let pkeybindings = pkey
                .columns
                .iter()
                .map(|col| {
                    let field_name = syn::Ident::new(col, proc_macro2::Span::call_site());

                    quote! {
                           .bind(v.#field_name)
                    }
                })
                .collect::<Vec<_>>();

            let inner_fields = columns
                .iter()
                .filter(|col| !cols.contains(&col.name))
                .map(|col| {
                    let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());
                    quote! {
                           .bind(json.#field_name.unwrap())
                    }
                });

            let select = "SELECT ".to_owned()
                + &cols.to_owned()
                + " FROM "
                + &table_name.to_owned()
                + " WHERE "
                + &where_cols;
            let del = "DELETE FROM ".to_owned() + &table_name.to_owned() + " WHERE " + &where_cols;

            let get =
                "get_".to_owned() + &table_name.to_lowercase() + "_by_" + &pkey_cols.join("_");

            let get_handler_function_name = get + "_handler";
            let test_get_handler_function_name = get_handler_function_name.clone() + "_test";

            let get_handler_function_name_syn =
                syn::Ident::new(&get_handler_function_name, proc_macro2::Span::call_site());

            //let key_syn = syn::Ident::new(&route, proc_macro2::Span::call_site());
            let get_handler = quote! {

            #[get(#route)]
            async fn #get_handler_function_name_syn(path: web::Path<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {
                    println!("{}", #select);
                    let v = path.into_inner();
                let res: Vec<#struct_name> = sqlx::query_as::<_,#struct_name>(#select)#(#pkeybindings)*.fetch_all( pool.get_ref()).await.unwrap();

                    match res {
                        Ok(a) => HttpResponse::Ok().insert_header("Content-type", "application/json").json(res),
                        Err(e) => {
                            eprint!("Query failed: {}, {}",#select,e);
                            HttpResponse::InternalServerError().body("Internal server error")
                        }
                    }
            }


                };

            let test_get_handler = quote! {

            #[cfg(test)]

                                           #[actix_rt::test]
                                           async fn #test_get_handler_function_name() {
                           let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                                                   let pool = PgPool::connect(database_url).await.expect("Failed to create pool.");
                                               let app = test::init_service(
                                                   App::new().app_data( web::Data::new(pool.clone())).service(web::scope("").service(#get_handler_function_name_syn) )).await;
                                               let req = test::TestRequest::get().uri(#route).to_request();
                                               let resp = test::call_service(&app, req).await;

                                               assert_eq!(resp.status(), StatusCode::OK);

                                               let body = test::read_body(resp).await;
                                           }




                };

            testoutput.extend(test_get_handler);
            let delete =
                "delete_".to_owned() + &table_name.to_lowercase() + "_by_" + &pkey_cols.join("_");
            let delete_handler_function_name = delete + "_handler";

            let delete_handler_function_name_syn = syn::Ident::new(
                &delete_handler_function_name,
                proc_macro2::Span::call_site(),
            );

            let delete_handler = quote! {
            #[delete(#route)]
            async fn #delete_handler_function_name_syn(path: web::Path<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {
                    println!("{}", #del);
                    let v = path.into_inner();
                let res = sqlx::query(#del)#(#pkeybindings)*.execute( pool.get_ref()).await.unwrap();
                let mut response = HttpResponse::Ok();
                //response.insert_header(("Content-Type", "application/json"));
                //response.json(res)
                response
            }



                };

            output.extend(get_handler);
            output.extend(delete_handler);

            let update = "update_".to_owned() + &table_name.to_lowercase();
            let update_handler_function_name = update + "_handler";

            let update_handler_function_name_syn = syn::Ident::new(
                &update_handler_function_name,
                proc_macro2::Span::call_site(),
            );

            let pkey_inlin = quote! {
                vec![#(#pkey_cols),*]
            };

            let update_handler = quote! {
            #[patch(#route)]
            async fn #update_handler_function_name_syn(path: web::Path<#struct_name>, json: web::Json<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {

            let pkeys = #pkey_inlin;
            let active_fields : Vec<(&str, &dyn std::fmt::Debug)>= json.non_null_fields();
            let fields_length  = active_fields.len();
            let update_where_cols = pkeys
                   .iter()
                   .enumerate()
                   .map(|(index, item)| format!("{} = {}", item, index + 1 + fields_length))
                   .collect::<Vec<String>>()
                   .join(" AND ");

            let insert_sql = "UPDATE ".to_owned() + &#table_name.to_owned()
                             +" set " + &active_fields.into_iter().enumerate().filter(|(index, (name, value)) | !pkeys.contains(name) ). map(|(index,(name,value)) | format!(" {} = ${} ", &name.to_string(), index+2)).collect::<Vec<_>>().join(" ").to_owned()
                            + " where "
                            + &update_where_cols;
            println!("{}",insert_sql);
            let v= path.into_inner();
            let mut sqlx_query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> = sqlx::query(&insert_sql)#(#pkeybindings)* #(#inner_fields)*;
            let result = sqlx_query.execute( pool.get_ref()).await;
            match result {
                Ok(res) => { println!("Query executed successfully: {:?}", res); }
                Err(e) => {  println!("Error executing query: {:?}", e); } }
            let mut response = HttpResponse::Ok();
            response
            }
            };
            output.extend(update_handler);
        }
        let select = "SELECT ".to_owned() + &cols.to_owned() + " FROM " + &table_name.to_owned();

        let get = "get_".to_owned() + &table_name.to_lowercase();

        let getfunctionname = syn::Ident::new(&get, proc_macro2::Span::call_site());
        let route = "/".to_owned() + table_name;
        let get_handler_function_name = "get_".to_owned() + &table_name.to_lowercase() + "_handler";
        let test_get_handler_function_name = "test_get_".to_owned() + &table_name.to_lowercase();
        let test_get_handler_function_name_syn = syn::Ident::new(
            &test_get_handler_function_name,
            proc_macro2::Span::call_site(),
        );
        let get_handler_function_name_syn =
            syn::Ident::new(&get_handler_function_name, proc_macro2::Span::call_site());
        let get_handler = quote! {
                             #[get(#route)]
                             async fn #get_handler_function_name_syn(pool: web::Data<PgPool>) -> impl Responder {
        let res: Result<Vec<#struct_name>, sqlx::Error> =  sqlx::query_as::<_,#struct_name>(#select).fetch_all(pool.get_ref()).await;


                match res {
                                         Ok(a) => {
        let mut response = HttpResponse::Ok();
                                response.insert_header(("content-type", "application/json"));
                                response.json(a)


                            },
                                         Err(e) => {
                                             eprint!("Query failed: {}, {}",#select,e);
                                             HttpResponse::InternalServerError().body("Internal server error")
                                         }
                                     }


                             }
        };

        let get_test_handler = quote! {

                           #[actix_rt::test]
                           async fn #test_get_handler_function_name_syn() {
           let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                                   let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");
                               let app = test::init_service(
                                   App::new().app_data(web::Data::new(pool.clone())).service(web::scope("").service(#get_handler_function_name_syn) )).await;
                               let req = test::TestRequest::get().uri(#route).to_request();
                               let resp = test::call_service(&app, req).await;

                let status = resp.status().clone();
                               let body = test::read_body(resp).await;
                                println!("{:?}", body.clone());
                               assert_eq!(status, StatusCode::OK);

                           }






        };

        testoutput.extend(get_test_handler);
        let post_handler_function_name =
            "post_".to_owned() + &table_name.to_lowercase() + "_handler";
        let post_handler_function_name_syn =
            syn::Ident::new(&post_handler_function_name, proc_macro2::Span::call_site());
        let post_handler = quote! {
        #[post(#route)]
        async fn #post_handler_function_name_syn(record: web::Json<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {

                let json = serde_json::to_value(record).unwrap();
                let fields: Vec<&str> = json.as_object().unwrap().keys().map(|s| s.as_str()) .collect();
                let placeholders: Vec<String> = (1..=fields.len()).map(|i| format!("${}", i)).collect();
                let values: Vec<&serde_json::Value> = json.as_object().unwrap().values().collect();
                let query = format!( "INSERT INTO {} ({}) VALUES ({})", #table_name, fields.join(", "), placeholders.join(", ") );
                let mut query_builder = sqlx::query(&query);
                for (i, value) in values.iter().enumerate() {
                    query_builder = match value {
                        serde_json::Value::String(s) => query_builder.bind(s),
                        serde_json::Value::Number(n) if n.is_i64() => query_builder.bind(n.as_i64().unwrap()),
                        serde_json::Value::Number(n) if n.is_f64() => query_builder.bind(n.as_f64().unwrap()),
                        serde_json::Value::Bool(b) => query_builder.bind(*b),
                        _ => query_builder,
                    };
                }

                println!("{}", "Processing data");
                query_builder.execute(pool.get_ref()).await;

            let mut response = HttpResponse::Ok();
            response.insert_header(("Content-Type", "application/json"));
                response.json(json)
        }
            };

        output.extend(new_struct);
        output.extend(new_struct2);
        output.extend(get_handler);
        output.extend(post_handler);
    }
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
    // TokenStream::from(output)
    TokenStream::new()
}

#[proc_macro]
pub fn generate_structs_from_file(attr: TokenStream) -> TokenStream {
    // Parse the attribute input for the file path
    let relative_path = parse_macro_input!(attr as syn::LitStr).value();

    // Construct the absolute path
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let file_path = PathBuf::from(manifest_dir).join(relative_path);

    // Read the JSON file
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");

    let openapi: OpenApiSpec =
        serde_json::from_str(&file_content).expect("Json was not well formatted");

    // Generate structs based on the JSON data
    let mut output = quote! {};

    for struct_def in openapi.components.unwrap().schemas.unwrap() {
        let struct_name = syn::Ident::new(&struct_def.0, proc_macro2::Span::call_site());
        let properties = struct_def.1.properties.unwrap();

        let fields = properties.iter().map(|field| {
            let field_name = syn::Ident::new(field.0, proc_macro2::Span::call_site());

            let field_type_str = field.1.type_.as_ref().unwrap().to_string();
            let field_ty: syn::Type = match field_type_str.as_str() {
                "string" => syn::parse_str("String").expect("Invalid type"),
                "integer" => syn::parse_str("i32").expect("Invalid type"),
                "boolean" => syn::parse_str("bool").expect("Invalid type"),
                // Handle other cases as needed
                _ => panic!("Unsupported type"),
            };

            quote! {
                pub #field_name: #field_ty,
            }
        });

        let new_struct = quote! {
            #[derive(Deserialize,Debug)]
            pub struct #struct_name {
                #(#fields)*
            }
        };

        output.extend(new_struct);
    }

    // TokenStream::from(output)
    //
    output.into()
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
