extern crate proc_macro;
mod select;
use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;
use syn::parse_macro_input;
use syn::LitStr;
#[derive(Debug)]
enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Float(f64),
    Vec(Vec<String>),
}
trait MapFields {
     fn map_columns_to_fields(&self) -> Vec<(syn::Type, Value, String, proc_macro2::Ident, String)>;
}
impl MapFields for parse::Table {
    fn map_columns_to_fields(
        &self
    ) -> Vec<(syn::Type, Value, String, proc_macro2::Ident, String)> {
        self.columns
            .iter()
            .map(|col| {
                let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());
                let field_name_str = col.name.clone();
                let field_type_str = col.dtype.to_string();
                match field_type_str.to_uppercase().as_str() {
                    "VARCHAR" => (
                        syn::parse_str::<syn::Type>("String").expect("Invalid type"),
                        Value::Str("Test".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "INT" => (
                        syn::parse_str::<syn::Type>("i32").expect("Invalid type"),
                        Value::Int(999),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "INTEGER" => (
                        syn::parse_str::<syn::Type>("i32").expect("Invalid type"),
                        Value::Int(999),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "BOOL" => (
                        syn::parse_str::<syn::Type>("bool").expect("Invalid type"),
                        Value::Bool(true),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "TEXT" => (
                        syn::parse_str::<syn::Type>("String").expect("Invalid type"),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "TEXT[]" => (
                        syn::parse_str::<syn::Type>("Vec<String>").expect("Invalid type"),
                        Value::Vec(vec!["test".to_string(), "test".to_string()]),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "DOUBLE" => (
                        syn::parse_str::<syn::Type>("f64").expect("Invalid type"),
                        Value::Float(999.0),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "UUID" => (
                        syn::parse_str::<syn::Type>("String").expect("Invalid type"),
                        Value::Str("acde070d-8c4c-4f0d-9d8a-162843c10333".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "CHARACTER" => (
                        syn::parse_str::<syn::Type>("String").expect("Invalid type"),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "TIMESTAMPTZ" => (
                        syn::parse_str::<syn::Type>("String").expect("Invalid type"),
                        Value::Str("2016-01-25 10:10:10.555555-05:00".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "TIMESTAMP" => (
                        syn::parse_str::<syn::Type>("String").expect("Invalid type"),
                        Value::Str("2024-10-16 14:30:00".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    "BOOLEAN" => (
                        syn::parse_str::<syn::Type>("bool").expect("Invalid type"),
                        Value::Bool(true),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                    _ => (
                        syn::parse_str::<syn::Type>("")
                            .unwrap_or_else(|_| panic!("Invalid type found {}", field_type_str)),
                        Value::Str("test".to_string()),
                        field_type_str,
                        field_name,
                        field_name_str,
                    ),
                }
            })
            .collect()
    }
    

}

impl Value {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Value::Int(val) => quote! { Some(#val), },
            Value::Str(val) => quote! { Some(String::from(#val)), },
            Value::Bool(val) => quote! { Some(#val), },
            Value::Float(val) => quote! { Some(#val), },
            Value::Vec(val) => {
                let v = val.iter().map(|x| quote! {#x.to_string(),});
                quote! {Some(vec![#(#v)*]),}
            }
        }
    }
    fn to_string(&self) -> String {
        match self {
            Value::Int(val) => val.to_string(),
            Value::Str(val) => String::from(val),
            Value::Bool(val) => val.to_string(),
            Value::Float(val) => val.to_string(),
            Value::Vec(val) => val.join(","),
        }
    }
}

trait RouteGenerator {
    fn generate_route(&self) -> proc_macro2::TokenStream;

    fn struct_name(&self) -> String;

    fn struct_identifier(&self) -> syn::Ident;
    fn method(&self) -> String;

    fn handler_function_name(&self) -> String;

    fn route(&self) -> String;

    fn generate_test(&self) -> proc_macro2::TokenStream;
}

struct GetAllRoute {
    table: parse::Table,
}

struct GetRoute {
    table: parse::Table,
}

struct PostRoute {
    table: parse::Table,
}

struct DeleteRoute {
    table: parse::Table,
}

struct UpdateRoute {
    table: parse::Table,
}

impl RouteGenerator for UpdateRoute {
    fn method(&self) -> String {
        String::from("PATCH")
    }

    fn struct_name(&self) -> String {
        self.table.name.clone()
    }
    fn struct_identifier(&self) -> syn::Ident {
        syn::Ident::new(&to_camel_case(self.struct_name().as_str()), proc_macro2::Span::call_site())
    }

    fn handler_function_name(&self) -> String {
        format!(
            "{}_{}_by_{}_handler",
            self.method().to_lowercase(),
            self.struct_name().to_lowercase(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>()
                .join("_and_")
        )
    }
    fn route(&self) -> String {
        format!(
            "/{}/{}",
            self.struct_name(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| format!("{{{}}}", x.name))
                .collect::<Vec<String>>()
                .join("/")
        )
    }
    fn generate_route(&self) -> proc_macro2::TokenStream {
        let update_handler_function_name_syn = syn::Ident::new(
            &self.handler_function_name(),
            proc_macro2::Span::call_site(),
        );

        let struct_name = self.struct_identifier();
        let route = self.route();
        quote! {
            #[patch(#route)]
            async fn #update_handler_function_name_syn(path: web::Path<#struct_name>, json: web::Json<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {
                let v = path.into_inner();
                let p = pool.get_ref();
                let j = json.into_inner();
                let result = #struct_name::update(p, j,v).await;
                match result {
                    Ok(res) => { println!("Query executed successfully: {:?}", res); }
                    Err(e) => {  println!("Error executing query: {:?}", e); } }
                let response = HttpResponse::Ok();
                response
            }
        }
    }

    fn generate_test(&self) -> proc_macro2::TokenStream {
        let test_post_handler_function_name_syn = syn::Ident::new(
            format!("test_{}", self.handler_function_name()).as_str(),
            proc_macro2::Span::call_site(),
        );
        let post_handler_function_name_syn = &syn::Ident::new(
            self.handler_function_name().as_str(),
            proc_macro2::Span::call_site(),
        );
        let struct_name = self.struct_identifier();
        let route = self.route();

        let fieldst = self.table.map_columns_to_fields();

        let constructors = generate_default_constructors(&fieldst);

        quote! {
            #[actix_rt::test]
            async fn #test_post_handler_function_name_syn() {
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");
                let app = test::init_service(
                    App::new().app_data(web::Data::new(pool.clone())).service(web::scope("").service(#post_handler_function_name_syn))
                ).await;

                let test_data = #struct_name {
                    #(#constructors)*
                };

                let req = test::TestRequest::post()
                    .uri(#route)
                    .set_json(&test_data).append_header(("Content-type", "application/json"))
                    .to_request();
                let resp = test::call_service(&app, req).await;

                let status = resp.status().clone();
                let body = test::read_body(resp).await;
                println!("{:?}", body.clone());
                assert_eq!(status, StatusCode::OK);
            }
        }
    }
}

impl RouteGenerator for PostRoute {
    fn method(&self) -> String {
        String::from("POST")
    }

    fn struct_name(&self) -> String {
        self.table.name.clone()
    }
    fn struct_identifier(&self) -> syn::Ident {
        syn::Ident::new(&to_camel_case(self.struct_name().as_str()), proc_macro2::Span::call_site())
    }

    fn handler_function_name(&self) -> String {
        format!(
            "{}_{}_by_{}_handler",
            self.method().to_lowercase(),
            self.struct_name().to_lowercase(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>()
                .join("_and_")
        )
    }
    fn route(&self) -> String {
        format!(
            "/{}/{}",
            self.struct_name(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| format!("{{{}}}", x.name))
                .collect::<Vec<String>>()
                .join("/")
        )
    }
    fn generate_route(&self) -> proc_macro2::TokenStream {
        let post_handler_function_name_syn = syn::Ident::new(
            &self.handler_function_name(),
            proc_macro2::Span::call_site(),
        );
        let struct_name = self.struct_identifier();

        let route = self.route();
        quote! {
               #[post(#route)]
               async fn #post_handler_function_name_syn(json: web::Json<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {
                let p = pool.get_ref();
            let v =   json.into_inner();
            let result = #struct_name::insert(p,v ).await;

                  match result {
                       Ok(_a) => {
                            let mut response = HttpResponse::Ok();
                   response.insert_header(("Content-Type", "application/json"));
                   response.json("{\"message\":\"okay\"}")

                       }
                       Err(e) =>
                       {
        eprint!("Unexpected error: {} ",e);
                           HttpResponse::InternalServerError().body("Internal server error")

                       }
                   }

                          }
           }
    }

    fn generate_test(&self) -> proc_macro2::TokenStream {
        let test_post_handler_function_name_syn = syn::Ident::new(
            format!("test_{}", self.handler_function_name()).as_str(),
            proc_macro2::Span::call_site(),
        );
        let post_handler_function_name_syn = &syn::Ident::new(
            self.handler_function_name().as_str(),
            proc_macro2::Span::call_site(),
        );
        let struct_name = self.struct_identifier();
        let route = self.route();

        let fieldst = self.table.map_columns_to_fields();

        let constructors = generate_default_constructors(&fieldst);

        quote! {
            #[actix_rt::test]
            async fn #test_post_handler_function_name_syn() {
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");
                let app = test::init_service(
                    App::new().app_data(web::Data::new(pool.clone())).service(web::scope("").service(#post_handler_function_name_syn))
                ).await;

                let test_data = #struct_name {
                    #(#constructors)*
                };

                let req = test::TestRequest::post()
                    .uri(#route)
                    .set_json(&test_data).append_header(("Content-type", "application/json"))
                    .to_request();
                let resp = test::call_service(&app, req).await;

                let status = resp.status().clone();
                let body = test::read_body(resp).await;
                println!("{:?}", body.clone());
                assert_eq!(status, StatusCode::OK);
            }
        }
    }
}
impl RouteGenerator for DeleteRoute {
    fn method(&self) -> String {
        String::from("DELETE")
    }

    fn struct_name(&self) -> String {
        self.table.name.clone()
    }
    fn struct_identifier(&self) -> syn::Ident {
        syn::Ident::new(&to_camel_case(self.struct_name().as_str()), proc_macro2::Span::call_site())
    }

    fn handler_function_name(&self) -> String {
        format!(
            "{}_{}_by_{}_handler",
            self.method().to_lowercase(),
            self.struct_name().to_lowercase(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>()
                .join("_and_")
        )
    }
    fn route(&self) -> String {
        format!(
            "/{}/{}",
            self.struct_name(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| format!("{{{}}}", x.name))
                .collect::<Vec<String>>()
                .join("/")
        )
    }

    fn generate_route(&self) -> proc_macro2::TokenStream {
        let delete_handler_function_name_syn = syn::Ident::new(
            self.handler_function_name().as_str(),
            proc_macro2::Span::call_site(),
        );
        let route = self.route();
        let struct_name = self.struct_identifier();
        quote! {
            #[delete(#route)]
            async fn #delete_handler_function_name_syn(path: web::Path<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {
                let v = path.into_inner();
                let p = pool.get_ref();
                let res = #struct_name::delete(p, v).await;

        match res {
                    Ok(_a) => {
                        HttpResponse::Ok() .content_type("application/json") .body(r#"{"message": "Succesfully deleted."}"#)
                    },
                    Err(e) => {
                        eprint!("Query failed: {}",e);
                        HttpResponse::InternalServerError().body("Internal server error")
                    }
                }



            }
        }
    }

    fn generate_test(&self) -> proc_macro2::TokenStream {
        let test_delete_handler_function_name_syn = syn::Ident::new(
            format!("test_{}", self.handler_function_name()).as_str(),
            proc_macro2::Span::call_site(),
        );
        let delete_handler_function_name_syn = syn::Ident::new(
            self.handler_function_name().as_str(),
            proc_macro2::Span::call_site(),
        );

        let fields_t = self.table.map_columns_to_fields();

        let pkey = generate_default_pkey(
            &fields_t,
            &self
                .table
                .primary_keys()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>(),
        );
        let route = format!("/{}/{}", &self.table.name, pkey.join("/"));

        quote! {
            #[actix_rt::test]
            async fn #test_delete_handler_function_name_syn() {
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");
                let app = test::init_service(
                    App::new().app_data(web::Data::new(pool.clone())).service(web::scope("").service(#delete_handler_function_name_syn))
                ).await;

                println!("{}", #route);
                // Create the test request with the JSON payload
                let req = test::TestRequest::delete().uri(#route).append_header(("Content-type", "application/json")).to_request();
                let resp = test::call_service(&app, req).await;

                let status = resp.status().clone();
                let body = test::read_body(resp).await;
                println!("{:?}", body.clone());
                assert_eq!(status, StatusCode::OK);
            }
        }
    }
}

impl RouteGenerator for GetRoute {
    fn method(&self) -> String {
        String::from("GET")
    }
    fn struct_identifier(&self) -> syn::Ident {
        syn::Ident::new(&to_camel_case(self.struct_name().as_str()), proc_macro2::Span::call_site())
    }

    fn struct_name(&self) -> String {
        self.table.name.clone()
    }

    fn handler_function_name(&self) -> String {
        format!(
            "{}_{}_by_{}_handler",
            self.method().to_lowercase(),
            self.struct_name().to_lowercase(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>()
                .join("_and_")
        )
    }
    fn route(&self) -> String {
        format!(
            "/{}/{}",
            self.struct_name(),
            self.table
                .primary_keys()
                .into_iter()
                .map(|x| format!("{{{}}}", x.name))
                .collect::<Vec<String>>()
                .join("/")
        )
    }

    fn generate_route(&self) -> proc_macro2::TokenStream {
        let get_handler_function_name_syn = syn::Ident::new(
            &self.handler_function_name(),
            proc_macro2::Span::call_site(),
        );

        let struct_name = self.struct_identifier();


        let route = self.route();
        quote! {
            #[get(#route)]
            async fn #get_handler_function_name_syn(path: web::Path<#struct_name>, pool: web::Data<PgPool>) -> impl Responder {
                let v = path.into_inner();
                let p = pool.get_ref();
                let res: Result<Vec<#struct_name>,sqlx::Error> =  #struct_name::select_where(p,v).await;

                match res {
                    Ok(a) => {let mut response = HttpResponse::Ok();
                        response.insert_header(("Content-type", "application/json"));
                        response.json(a)},
                    Err(e) => {
                        eprint!("Query failed: {}",e);
                        HttpResponse::InternalServerError().body("Internal server error")
                    }
                }
            }
        }
    }

    fn generate_test(&self) -> proc_macro2::TokenStream {
        let test_get_handler_function_name_syn = syn::Ident::new(
            format!("test_{}", &self.handler_function_name()).as_str(),
            proc_macro2::Span::call_site(),
        );
       // let route = self.route();
        let fieldst = self.table.map_columns_to_fields();


        let pkey = generate_default_pkey(
            &fieldst,
            &self
                .table
                .primary_keys()
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<String>>(),
        );
        let route = format!("/{}/{}", &self.table.name, pkey.join("/"));

        let handler_function_name_syn = syn::Ident::new(
            &self.handler_function_name(),
            proc_macro2::Span::call_site(),
        );        quote! {
            #[actix_rt::test]
            async fn #test_get_handler_function_name_syn() {
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");
                let app = test::init_service(
                    App::new().app_data(web::Data::new(pool.clone())).service(web::scope("").service(#handler_function_name_syn))
                ).await;
                let req = test::TestRequest::get().uri(#route).to_request();
                let resp = test::call_service(&app, req).await;

                let status = resp.status().clone();
                let body = test::read_body(resp).await;
                println!("{:?}", body.clone());
                assert_eq!(status, StatusCode::OK);
            }
        }
    }
}

impl RouteGenerator for GetAllRoute {
    fn method(&self) -> String {
        String::from("GET")
    }

    fn struct_name(&self) -> String {
        self.table.name.clone()
    }
    fn struct_identifier(&self) -> syn::Ident {
        syn::Ident::new(&to_camel_case(self.struct_name().as_str()), proc_macro2::Span::call_site())
    }

    fn handler_function_name(&self) -> String {
        format!(
            "{}_{}_handler",
            self.method().to_lowercase(),
            self.struct_name().to_lowercase()
        )
    }
    fn route(&self) -> String {
        format!("/{}", self.struct_name())
    }

    fn generate_route(&self) -> proc_macro2::TokenStream {
        let get_handler_function_name_syn = syn::Ident::new(
            &self.handler_function_name(),
            proc_macro2::Span::call_site(),
        );
        let route = self.route();
        let struct_name = self.struct_identifier();

        quote! {
                    #[get(#route)]
                    async fn #get_handler_function_name_syn(pool: web::Data<PgPool>) -> impl Responder {
        let res: Result<Vec<#struct_name>, sqlx::Error> = #struct_name::select(&pool).await;

                        match res {
                            Ok(a) => {
                                let mut response = HttpResponse::Ok();
                                response.insert_header(("content-type", "application/json"));
                                response.json(a)
                            },
                            Err(e) => {
                                eprint!("Query failed: {}",e);
                                HttpResponse::InternalServerError().body("Internal server error")
                            }
                        }
                    }
                }
    }

    fn generate_test(&self) -> proc_macro2::TokenStream {
        let test_get_handler_function_name_syn = syn::Ident::new(
            format!("test_{}", &self.handler_function_name()).as_str(),
            proc_macro2::Span::call_site(),
        );
        let route = self.route();
        let handler_function_name_syn = syn::Ident::new(
            &self.handler_function_name(),
            proc_macro2::Span::call_site(),
        );
        quote! {
            #[actix_rt::test]
            async fn #test_get_handler_function_name_syn() {
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");
                let app = test::init_service(
                    App::new().app_data(web::Data::new(pool.clone())).service(web::scope("").service(#handler_function_name_syn))
                ).await;
                let req = test::TestRequest::get().uri(#route).to_request();
                let resp = test::call_service(&app, req).await;

                let status = resp.status().clone();
                let body = test::read_body(resp).await;
                println!("{:?}", body.clone());
                assert_eq!(status, StatusCode::OK);
            }
        }
    }
}


#[proc_macro]
pub fn generate_structs_from_ddl(attr: TokenStream) -> TokenStream {
    
    let relative_path = parse_macro_input!(attr as LitStr).value();
    let select_trait = quote! {

    use async_trait::async_trait;

    #[async_trait]
    pub trait Select {
        async fn select(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error>
        where
            Self: Sized;
    }
    };

    let insert_trait = quote! {
    #[async_trait]
    trait Insert {
        async fn insert(pool: &PgPool, value: Self) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error>
        where
            Self: Sized;
    }
    };
    let select_where_trait = quote! {
    #[async_trait]
    trait SelectWhere {
        async fn select_where(pool: &PgPool, value: Self) -> Result<Vec<Self>, sqlx::Error>
        where
            Self: Sized;
    }
    };
    let delete_trait = quote! {
    #[async_trait]
    trait Delete {
        async fn delete(pool: &PgPool, v: Self) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error>
        where
            Self: Sized;
    }
    };
    let update_trait = quote! {
    #[async_trait]
    trait Update {
        async fn update(pool: &PgPool, value: Self, v:Self) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error>
        where
            Self: Sized;
    }
    };

    let mut output = quote! {};
    output.extend(select_trait);
    output.extend(insert_trait);
    output.extend(select_where_trait);
    output.extend(delete_trait);
    output.extend(update_trait);
    let mut testoutput = quote! {};

    let tables = parse::parse_ddl_file(relative_path);
       for stmt in tables {
        let ddl = stmt.clone();
        let table = &ddl.0.clone();
        let struct_name = syn::Ident::new(&ddl.0.name, proc_macro2::Span::call_site());


        let fieldst = table.map_columns_to_fields();
        let fields = generate_fields(&fieldst);

        let struc = generate_struct(&struct_name, fields);

        output.extend(struc);

        let implementation = generate_impl(table);

        output.extend(implementation);
        let get_route_ = GetRoute {
            table: table.clone(),
        };
        let get_route = get_route_.generate_route();
        output.extend(get_route);
        let test_get_route = get_route_.generate_test();
        testoutput.extend(test_get_route);

        let update_route_ = UpdateRoute {
            table: table.clone(),
        };
        let update_route = update_route_.generate_route();

        output.extend(update_route);

        let get_handler_ = GetAllRoute {
            table: ddl.0.clone(),
        };
        let get_handler = get_handler_.generate_route();

        output.extend(get_handler.clone());

        let test_get_handler = get_handler_.generate_test();

        testoutput.extend(test_get_handler);

        let post_handler_ = PostRoute {
            table: ddl.0.clone(),
        };
        let post_handler = post_handler_.generate_route();
        output.extend(post_handler.clone());

        let test_post_handler = post_handler_.generate_test();
        testoutput.extend(test_post_handler);

        let delete_handler_ = DeleteRoute {
            table: ddl.0.clone(),
        };
        let delete_handler = delete_handler_.generate_route();
        output.extend(delete_handler);

        let test_delete_handler = delete_handler_.generate_test();
        testoutput.extend(test_delete_handler);

    }

    write_output_to_file(&output, &testoutput);

    TokenStream::new()
}
fn generate_impl(table: &parse::Table) -> proc_macro2::TokenStream {
    let tname = table.name.as_str();
    let struct_name = syn::Ident::new(&to_camel_case(tname).as_str(), proc_macro2::Span::call_site());
    let active_fields = table.columns.iter().map(|col| {
        let colname = &col.name;
        let field_name = syn::Ident::new(colname, proc_macro2::Span::call_site());
        quote! {
            if !self.#field_name.is_none(){
                fields.push((#colname, &self.#field_name as &dyn std::fmt::Debug) );
            }
        }
    });
    let active_fields_ = table.columns.iter().map(|col| {
        let colname = &col.name;
        let field_name = syn::Ident::new(colname, proc_macro2::Span::call_site());
        quote! {
            if !self.#field_name.is_none(){
                fields.push(#colname );
            }
        }
    });

    let table_name = &table.name;
    let insertable_fields = table
        .columns
        .iter()
        .filter(|x| x.default.is_none())
        .map(|col| col.name.clone())
        .collect::<Vec<_>>();
    let non_default_bindings: Vec<proc_macro2::TokenStream> = table
        .columns
        .iter()
        .filter(|x| x.default.is_none())
        .map(|col| {
            let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());
            let dtype = col.dtype.as_str();

            match dtype {
                "uuid" => quote! {
                    .bind( Uuid::parse_str(value.#field_name.as_ref().unwrap()).unwrap()   )
                },
                _ => quote! {
                    .bind(value.#field_name.as_ref().unwrap() )
                },
            }
        })
        .collect();
    let primary_keys: Vec<String> = table
        .constraints
        .iter()
        .filter_map(|item| {
            if let parse::Constraint::PrimaryKey(a) = item {
                Some(a)
            } else {
                None
            }
        })
        .flat_map(|x| x.columns.clone())
        .collect();
    let inner_field_bindings = table
        .columns
        .iter()
        .filter(|y| !primary_keys.contains(&y.name))
        .map(|col| {
            let colname = &col.name;
            let field_name = syn::Ident::new(colname, proc_macro2::Span::call_site());
            let dtype = col.dtype.as_str();
        
                
 match dtype {
                "uuid" => quote! {

                    if value.#field_name.is_some() {sqlx_query.push_bind( Uuid::parse_str(value.#field_name.as_ref().unwrap()).unwrap()   ); }
                },
                _ => quote! {
                   if value.#field_name.is_some(){ sqlx_query.push_bind(value.#field_name.as_ref().unwrap() );}
                },
            }
                
            
        });

    let pkeybindings: Vec<proc_macro2::TokenStream> = table
        .columns
        .iter()
        .filter(|x| primary_keys.contains(&x.name))
        .map(|col| {
            let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());
            let dtype = col.dtype.clone();

            match dtype.as_str() {
                "uuid" => quote! {
                    .bind( Uuid::parse_str(v.#field_name.as_ref().unwrap()).unwrap()   )
                },
                _ => quote! {
                    .bind(v.#field_name.as_ref().unwrap() )
                },
            }
        })
        .collect();
let pkey_builder_bindings: Vec<proc_macro2::TokenStream> = table
        .columns
        .iter()
        .filter(|x| primary_keys.contains(&x.name))
        .map(|col| {
            let field_name = syn::Ident::new(&col.name, proc_macro2::Span::call_site());
            let dtype = col.dtype.clone();

            match dtype.as_str() {
                "uuid" => quote! {
                    sqlx_query.push_bind( Uuid::parse_str(v.#field_name.as_ref().unwrap()).unwrap()   )
                },
                _ => quote! {
                    sqlx_query.push_bind(v.#field_name.as_ref().unwrap() )
                },
            }
        })
        .collect();


    let fields_length = insertable_fields.len();
    let insertable_fields_string = insertable_fields.join(",");

    let cols = table
        .columns
        .iter()
        .map(|col| {
            let binding = col.dtype.to_lowercase();
            let dtype = binding.as_str();
            match dtype {
                "uuid" => format!("cast({} as varchar) as {}", col.colname, col.colname),
                "timestamptz" => format!("cast({} as varchar) as {}", col.colname, col.colname),
                //"text[]" => format!("array_to_string({}, ',') as {}", col.colname, col.colname),
                "timestamp" => format!("cast({} as varchar) as {}", col.colname, col.colname),
                _ => col.colname.clone(),
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    let select = "SELECT ".to_owned() + &cols + " FROM " + table_name;

    let where_cols = primary_keys
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{} = ${}", item, index + 1))
        .collect::<Vec<String>>()
        .join(" AND ");

    let pkey_inlin = quote! {
        vec![#(#primary_keys),*]
    };
    let select_where =
        "SELECT ".to_owned() + &cols + " FROM " + table_name + " WHERE " + &where_cols;

    quote! {
                   impl #struct_name {
                       pub fn non_null_fields(&self) -> Vec<(&str, &dyn std::fmt::Debug)>{
                           let mut fields = Vec::new();
                           #(#active_fields)*
                           fields
                       }
                       
                       pub fn non_null_field_names(&self) -> Vec<&str>{
                        let mut fields = Vec::new();
                        #(#active_fields_)*
                        fields
                    }


                              }
        #[async_trait]
                   impl Delete for #struct_name {

                   async fn delete(pool: &PgPool, v: Self) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {

                            let query = format!("DELETE FROM {} WHERE {}", #table_name, #where_cols ) ;
                           let sqlx_query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> = sqlx::query(&query) #(#pkeybindings)*;

                           sqlx_query.execute(pool).await
                       }


                   }
    #[async_trait]
                   impl Update for #struct_name {

                   async fn update(pool: &PgPool, value: Self, v: Self) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {

            let pkeys = #pkey_inlin;
                let active_fields : Vec<&str>= value.non_null_field_names();
                let fields_length  = active_fields.len();
                let update_where_cols = pkeys
                    .iter()
                    .enumerate()
                    .map(|(index, item)| format!("{} = {}", item, index + 1 + fields_length))
                    .collect::<Vec<String>>()
                    .join(" AND ");

                let update_sql = "UPDATE ".to_owned() + &#table_name.to_owned()
                    +" set " + &active_fields.into_iter().enumerate().filter(|(_index, name) | !pkeys.contains(name) ). map(|(index,name) | format!(" {} = ${} ", &name.to_string(), index+2)).collect::<Vec<_>>().join(" ").to_owned()
                    + " where "
                    + &update_where_cols;
                println!("{}",update_sql);
                let mut sqlx_query = sqlx::QueryBuilder::new(&update_sql);
                #(#pkey_builder_bindings);* ;
                //let sqlx_query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> = sqlx::query(&update_sql)#(#pkeybindings)*;
                #(#inner_field_bindings)*;
                let final_query = sqlx_query.build();
                let result = final_query.execute(pool).await;
                result

                       }


                   }


                   #[async_trait]
                   impl Insert for #struct_name {

                   async fn insert(pool: &PgPool, value: Self) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
                           let placeholders: Vec<String> = (1..=#fields_length ).map(|i| format!("${}", i)).collect();

                            let query = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING *", #table_name, #insertable_fields_string, placeholders.join(", "));
                           let sqlx_query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> = sqlx::query(&query) #(#non_default_bindings)*;

                           sqlx_query.fetch_all(pool).await
                       }


                   }
            #[async_trait]
                   impl SelectWhere for #struct_name {

                       async fn select_where( pool: &PgPool, v: Self) -> Result<Vec<Self>, sqlx::Error> {

                        let rows: Result<Vec<#struct_name>,sqlx::Error> = sqlx::query_as::<_,#struct_name>(#select_where)#(#pkeybindings)*.fetch_all( pool ).await;


                           rows
                       }


                   }

                   #[async_trait]
                   impl Select for #struct_name {

                       async fn select( pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
                           let rows = sqlx::query_as::<_, #struct_name>( #select ) .fetch_all(pool) .await;
                           rows
                       }


                   }
               }
}


fn generate_fields<T: quote::ToTokens>(
    fieldst: &[(T, Value, String, proc_macro2::Ident, String)],
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fieldst.iter().map(|(field_ty, _, _, field_name, _)| {
        quote! {
            pub #field_name: Option<#field_ty>,
        }
    })
}
fn generate_default_constructors<T: quote::ToTokens>(
    fieldst: &[(T, Value, String, proc_macro2::Ident, String)],
) -> Vec<proc_macro2::TokenStream> {
    fieldst
        .iter()
        .map(|(_, value, _, field_name, _)| {
            let tokens = value.to_tokens();
            quote! {#field_name: #tokens }
        })
        .collect()
}
fn generate_default_pkey<T: quote::ToTokens>(
    fieldst: &[(T, Value, String, proc_macro2::Ident, String)],
    pkey_field_names: &[String],
) -> Vec<String> {
    fieldst
        .iter()
        .filter(|(_, _, _, _, field_name)| pkey_field_names.contains(field_name))
        .map(|(_, value, _, _, _field_name)| value.to_string())
        .collect()
}

fn generate_struct(
    struct_name: &syn::Ident,
    fields: impl Iterator<Item = proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        #[derive(Deserialize,Serialize,Debug,sqlx::FromRow,Clone)]
        pub struct #struct_name {
            #(#fields)*
        }
    }
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
fn to_camel_case(s: &str) -> String { let mut result = String::new(); let mut capitalize_next = false; for (i, word) in s.split_whitespace().enumerate() { if i > 0 { capitalize_next = true; } for (j, c) in word.chars().enumerate() { if j == 0 && capitalize_next { result.push(c.to_ascii_uppercase()); } else { result.push(c.to_ascii_lowercase()); } } } result }
