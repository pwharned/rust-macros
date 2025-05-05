use actix_files as fs;
use actix_files::NamedFile;
use actix_web::HttpRequest;
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use macros::{generate_structs_from_ddl, register_handlers, from_view};
use schemars::{schema_for, JsonSchema};
use serde::Serialize;
use serde::{Deserialize, Deserializer};
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};
use std::collections::HashMap;
use std::env;
use actix_cors::Cors;
use std::path::PathBuf;
generate_structs_from_ddl!("../server/schema.sql");
register_handlers!(r#"../server/generated_handlers.rs"#);
include!("../generated_handlers.rs");

from_view!("create view fields select cast(e.eid as text) as _asset_id, av.value as value, a.name as name FROM public.entityattributes e join public.attributevalues av on av.id = e.vid join public.attributes a on a.id = e.aid;" );

from_view!("create view parent_child select a.name as child, b.name as parent from attributes a join parent c on c.caid = a.id join parent p on p.paid = c.paid join attributes b on b.id = p.paid ;");

from_view!("create view parent_child_relation select aa.name as parent_attribute, aaa.name as child_attribute, a.value as parent_value, a2.value as child_value from relationship r join attributevalues a on a.id = r.pavid join attributevalues a2 on a2.id = r.cavid join attributes aa on aa.id = r.paid join attributes aaa on aaa.id = r.caid;");
from_view!("create view joined_attributes select a.name as name, av.value as value, av.id as id, a.id as aid from attributevalues av join attributes a on a.id = av.aid;"); 

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AssetAttribute {
    pub asset: assets,
    fields: Option<Vec<fields>>,
}

#[async_trait]
impl Select for AssetAttribute {
    async fn select(pool: &PgPool, v: Self) -> Result<Vec<Self>, sqlx::Error> {
        let assets = assets::select(pool, v.asset).await?;
        let asset_attribute = fields::select_all(pool).await?;
        let mut asset_attribute_hash: HashMap<String, Vec<fields>  > =
            HashMap::new();
        for attribute in asset_attribute {
            let asset_id  = attribute._asset_id.clone().unwrap();
            asset_attribute_hash.entry(asset_id).and_modify(|e| {e.insert(0, attribute)} ).or_insert(Vec::new());

        }

        let mut result = Vec::new();

        for asset in &assets {
            let _asset = asset.clone();
            let a = AssetAttribute {
                asset: _asset.clone(),
                fields: 
                    asset_attribute_hash
                        .get(&_asset.asset_id.unwrap()).cloned(),
                
            };
            result.push(a);
        }

        return Ok(result);
    }
}

#[async_trait]
impl Update for AssetAttribute {
    async fn update(pool: &PgPool, value: Self, v: Self) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error>
{
        let asset = value.asset.clone();
        let asset_id = asset.asset_id.clone();
        let update_response = assets::update(pool, asset, v.asset).await?;
        let fields = value.fields.unwrap();
//update entityattributes e set vid =  (select av.id from  attributevalues av join attributes a on av.aid = a.id  where a.name = 'Type' and av.value = 'Demo') from attributevalues av  join attributes a on a.id = av.aid where e.eid =  '00333df0-7dc2-4c2f-ae92-0fe4bdac7836' and a.name = 'Type' and av.value = 'Solution Templates' and av.id = e.vid and e.aid = a.id;
            let sql = r#"
        UPDATE entityattributes e
        SET vid = (
            SELECT av.id
            FROM attributevalues av
            JOIN attributes a ON av.aid = a.id
            WHERE a.name = $1 AND av.value = $2
        )
        FROM attributevalues av
        JOIN attributes a ON a.id = av.aid
        WHERE e.eid = $3
          AND a.name = $4
          AND av.value = $5
          AND av.id = e.vid
          AND e.aid = a.id
    "#;
            for f in fields {
                let result = sqlx::query(sql ).bind(&f.name).bind(&f.value).bind(&asset_id ).bind(&f.name).bind(&f.value).execute(pool).await? ;
        }

        return Ok(update_response);
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

        let result2 =  assets::insert(pool, value.asset.clone()).await?;
        let row = result2.first().unwrap();
        let asset_id: String = row.try_get("asset_id").unwrap();

        let query = r#"INSERT INTO temp_table (asset_id, name, value) VALUES ($1,$2,$3 ); "#;
        //let mut query_builder : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::postgres :: PgArguments > = sqlx ::query(query);
        if value.fields.is_some() {
            for k in &value.fields.clone().unwrap() {
                let _ = sqlx::query(query)
                    .bind(asset_id.clone())
                    .bind(k.name.clone())
                    .bind(k.value.clone())
                    .execute(&mut *tx)
                    .await;
            }
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

        let length = match &value.fields.as_ref() {
            Some(v) => v.len(),
            _ => 0,
        };

        if rows_affected != length {
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
#[get("/api/schemas/assetAttributes")]
async fn get_asset_with_attributes_handler_schema() -> impl Responder {
    let b = schema_for!(AssetAttribute);
    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
}

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}

#[post("/api/assetAttributes")]
async fn post_asset_with_attributes_handler(
    json: web::Json<Vec<AssetAttribute>>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let p = pool.get_ref();
    let v = json.into_inner();
    let mut resvec: Vec<assets> = Vec::new();
    for asset_attribute in v {
        match AssetAttribute::insert(p, asset_attribute).await {
            Ok(rv) => {
                for a in rv{
                    let asset: assets = assets::from_row(&a).unwrap();
                    resvec.push(asset);
                }
             },
            Err(e) => {
                eprintln!("Unexpected error: {}", e);
                return HttpResponse::InternalServerError()
                    .body(format!("Internal server error {}", e));
            }
        }
    }

    let mut response = HttpResponse::Ok();
    response.insert_header(("Content-Type", "application/json"));
    response.json(resvec)
}

#[get("/health/ping")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}
#[get("/api/assetAttributes")]
async fn get_asset_with_attributes_handler(
    pool: web::Data<PgPool>,
    info: web::Query<assets>,
) -> impl Responder {
    let v = info.into_inner();
    let av = AssetAttribute {
        asset: v,
        fields: None,
    };
    let res: Result<Vec<AssetAttribute>, sqlx::Error> = AssetAttribute::select(&pool, av).await;

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
    HttpServer::new( move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()  // Restrict to your frontend's origin
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(register_all_handlers)
            .service(get_asset_with_attributes_handler)
            .service(post_asset_with_attributes_handler)
            .service(get_parent_child_handler)
            .service(get_asset_with_attributes_handler_schema)
            .service(get_parent_child_relation_handler)
            .service(get_joined_attributes_handler)
            .service(health)
            .service(fs::Files::new("/admin", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}



