use actix_files as fs;
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use macros::{generate_structs_from_ddl, register_handlers};
use serde::Serialize;
use serde::{Deserialize, Deserializer};
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};
use std::collections::HashMap;
use std::env;
generate_structs_from_ddl!("../server/schema.sql");
register_handlers!(r#"../server/generated_handlers.rs"#);
include!("../generated_handlers.rs");

//select a.asset_id, at.name, av.value from public.entityattributes e join public.assets a on e.eid = a.asset_id join public.attributevalues av on av.aid = e.aid join public.attributes at on at.id = av.aid limit 1;
#[derive(Deserialize, Serialize)]
pub struct AssetAttribute {
    pub asset: assets,
    fields: HashMap<String, String>,
}

#[async_trait]
impl Select for AssetAttribute {
    async fn select(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let assets = assets::select(pool).await?;
        let asset_attribute = sqlx :: query
            (
                "select cast(assets.asset_id as text) as asset_id, attributevalues.value,attributes.name from public.assets assets join entityattributes entityattributes on entityattributes.eid = assets.asset_id join public.attributevalues attributevalues on attributevalues.id = entityattributes.vid join public.attributes attributes on attributes.id = attributevalues.aid "          )
            .fetch_all(pool).await?;

        let mut asset_attribute_hash: HashMap<String, HashMap<String, String>> = HashMap::new();
        for attribute in asset_attribute {
            let asset_id: String = attribute.get("asset_id");
            let value: String = attribute.get("value");
            let name: String = attribute.get("name");

            asset_attribute_hash
                .entry(asset_id.clone())
                .or_insert_with(HashMap::new)
                .insert(name.clone(), value.clone());
        }

        let mut result = Vec::new();

        for asset in &assets {
            let _asset = asset.clone();
            let a = AssetAttribute {
                asset: _asset.clone(),
                fields: asset_attribute_hash
                    .get(&_asset.asset_id.unwrap())
                    .unwrap()
                    .clone(),
            };
            result.push(a);
        }

        return Ok(result);
    }
}

#[async_trait]
impl Insert for AssetAttribute {
    async fn insert(pool: &PgPool, value: Self) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let q1 = r#"CREATE TEMPORARY TABLE temp_table (
    asset_id text,
    name text,
    value text
);"#;
        let _result = sqlx::query(q1).execute(&mut *tx).await?;

        let result2 = assets::insert(pool, value.asset.clone()).await?;
        let row = result2.first().unwrap();
        let asset_uid: Uuid = row.try_get("asset_id").unwrap();
        let asset_id: String = asset_uid.to_string();

        let query = r#"INSERT INTO temp_table (asset_id, name, value) VALUES ($1,$2,$3 ); "#;
        //let mut query_builder : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::postgres :: PgArguments > = sqlx ::query(query);

        for (k, v) in &value.fields {
            let _ = sqlx::query(query)
                .bind(asset_id.clone())
                .bind(k)
                .bind(v)
                .execute(&mut *tx)
                .await;
        }

        let count = sqlx::query("Select cast(count(*) as text) as count from temp_table")
            .fetch_all(&mut *tx)
            .await?;
        let row2 = count.first().unwrap();
        let c: String = row2.try_get("count").unwrap();
        println!("{}", c);

        let q3 = format!("insert into entityattributes (vid, aid, eid) select attributevalues.id as vid,attributes.id as aid, '{}' as eid from attributevalues attributevalues join attributes attributes on attributes.id = attributevalues.aid join temp_table temp_table on temp_table.name = attributes.name and temp_table.value = attributevalues.value ;",asset_id);

        let result = sqlx::query(&q3).execute(&mut *tx).await?;

        let rows_affected = result.rows_affected() as usize;

        if rows_affected != value.fields.len() {
            tx.rollback().await?;
            let _delete = sqlx::query(&format!(
                "delete from assets where asset_id = '{}'",
                asset_id
            ))
            .execute(pool)
            .await?;
            let message = "One or more values violated a foreign key constraint";
            let e = Err(message);
            let db_error = sqlx::Error::Database(e.expect(message));
            return Err(db_error);
        }

        tx.commit().await?;
        Ok(result2)
    }
}

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}
#[post("/assetAttributes")]
async fn post_asset_with_attributes_handler(
    json: web::Json<AssetAttribute>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let p = pool.get_ref();
    let v = json.into_inner();
    let result = AssetAttribute::insert(p, v).await;
    match result {
        Ok(_a) => {
            let mut response = HttpResponse::Ok();
            response.insert_header(("Content-Type", "application/json"));
            response.json("{\"message\":\"okay\"}")
        }
        Err(e) => {
            eprint!("Unexpected error: {} ", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
#[get("/health/ping")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}
#[get("/assetAttributes")]
async fn get_asset_with_attributes_handler(pool: web::Data<PgPool>) -> impl Responder {
    let res: Result<Vec<AssetAttribute>, sqlx::Error> = AssetAttribute::select(&pool).await;

    match res {
        Ok(a) => {
            let mut response = HttpResponse::Ok();
            response.insert_header(("content-type", "application/json"));

            response.json(a)
        }
        Err(e) => {
            eprint!("Query failed: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(register_all_handlers)
            .service(get_asset_with_attributes_handler)
            .service(post_asset_with_attributes_handler)
            .service(health)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
