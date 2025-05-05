pub trait Schema { fn schema() -> String; } use async_trait :: async_trait;
#[async_trait] pub trait Select
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error > where Self : Sized;
} #[async_trait] pub trait SelectAll
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error > where Self : Sized;
} #[async_trait] trait Insert
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error > where Self : Sized;
} #[async_trait] trait SelectWhere
{
    async fn select_where(pool : & PgPool, value : Self) -> Result < Vec <
    Self > , sqlx :: Error > where Self : Sized;
} #[async_trait] trait Delete
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error > where Self : Sized;
} #[async_trait] trait Update
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error > where Self : Sized;
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct actions
{
    pub action_id : Option < String > , pub action_user : Option < String > ,
    pub created_at : Option < String > , pub related_asset : Option < String >
    , pub action_type : Option < String > ,
} impl Schema for actions
{
    fn schema() -> String
    {
        let schema = schema_for! (actions); serde_json ::
        to_string(& schema).unwrap()
    }
} impl actions
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.action_id.is_none()
        {
            fields.push(("action_id", & self.action_id as & dyn std :: fmt ::
            Debug));
        } if ! self.action_user.is_none()
        {
            fields.push(("action_user", & self.action_user as & dyn std :: fmt
            :: Debug));
        } if ! self.created_at.is_none()
        {
            fields.push(("created_at", & self.created_at as & dyn std :: fmt
            :: Debug));
        } if ! self.related_asset.is_none()
        {
            fields.push(("related_asset", & self.related_asset as & dyn std ::
            fmt :: Debug));
        } if ! self.action_type.is_none()
        {
            fields.push(("action_type", & self.action_type as & dyn std :: fmt
            :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.action_id.is_none()
        { fields.push("action_id"); } if ! self.action_user.is_none()
        { fields.push("action_user"); } if ! self.created_at.is_none()
        { fields.push("created_at"); } if ! self.related_asset.is_none()
        { fields.push("related_asset"); } if ! self.action_type.is_none()
        { fields.push("action_type"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.action_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "action_id", count));
        } if ! self.action_user.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "action_user", count));
        } if ! self.created_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "created_at", count));
        } if ! self.related_asset.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "related_asset", count));
        } if ! self.action_type.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "action_type", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for actions
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "actions", "action_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.action_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for actions
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["action_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "actions")); if
        value.action_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "action_id")).push_bind(Uuid ::
            parse_str(value.action_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.action_user.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "action_user")).push_bind(value.action_user.as_ref().unwrap());
            first = false;
        }; if value.created_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "created_at")).push_bind(value.created_at.as_ref().unwrap());
            first = false;
        }; if value.related_asset.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "related_asset")).push_bind(Uuid ::
            parse_str(value.related_asset.as_ref().unwrap()).unwrap()); first
            = false;
        }; if value.action_type.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "action_type")).push_bind(value.action_type.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "action_id")).push_bind(Uuid ::
        parse_str(v.action_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for actions
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < actions > , sqlx :: Error > = sqlx ::
        query_as :: < _, actions >
        ("SELECT cast(action_id as varchar) as action_id,action_user,cast(created_at as varchar) as created_at,cast(related_asset as varchar) as related_asset,action_type FROM actions WHERE action_id = $1").bind(Uuid
        ::
        parse_str(v.action_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for actions
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(action_id as varchar) as action_id,action_user,cast(created_at as varchar) as created_at,cast(related_asset as varchar) as related_asset,action_type FROM actions".to_string());
        println!
        ("{}",
        "SELECT cast(action_id as varchar) as action_id,action_user,cast(created_at as varchar) as created_at,cast(related_asset as varchar) as related_asset,action_type FROM actions");
        let final_query : sqlx :: query :: QueryAs < _, actions, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for actions
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(action_id as varchar) as action_id,action_user,cast(created_at as varchar) as created_at,cast(related_asset as varchar) as related_asset,action_type FROM actions".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.action_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "action_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.action_id.as_ref().unwrap()).unwrap());
        }; if value.action_user.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "action_user")); count += 1;
            sqlx_query.push_bind(value.action_user.as_ref().unwrap());
        }; if value.created_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "created_at")); count += 1;
            sqlx_query.push_bind(value.created_at.as_ref().unwrap());
        }; if value.related_asset.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "related_asset")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.related_asset.as_ref().unwrap()).unwrap());
        }; if value.action_type.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "action_type")); count += 1;
            sqlx_query.push_bind(value.action_type.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, actions, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for actions
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 3usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "actions",
        "action_user,related_asset,action_type", placeholders.join(", "),
        "cast(action_id as text) as action_id,action_user,cast(created_at as text) as created_at,cast(related_asset as text) as related_asset,action_type");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.action_user.as_ref().or(None)).bind(Uuid ::
        parse_str(value.related_asset.as_ref().unwrap()).unwrap()).bind(value.action_type.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/actions/{action_id}")] async fn
get_actions_by_action_id_handler(path : web :: Path < actions > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < actions > , sqlx :: Error > = actions :: select_where(p, v).await; match
    res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/actions/{action_id}")] async fn
patch_actions_by_action_id_handler(path : web :: Path < actions > , json : web
:: Json < actions > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = actions :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/actions")] async fn
get_actions_handler(info : web :: Query < actions > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < actions > , sqlx ::
    Error > = actions :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/actions")] async fn get_actions_handler_schema() -> impl
Responder
{
    let b = schema_for! (actions); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/actions")] async fn
post_actions_by_action_id_handler(json : web :: Json < actions > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = actions ::
    insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = actions :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/actions/{action_id}")] async fn
delete_actions_by_action_id_handler(path : web :: Path < actions > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = actions ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct asset_bookmarks
{ pub asset_id : Option < String > , pub email : Option < String > , } impl
Schema for asset_bookmarks
{
    fn schema() -> String
    {
        let schema = schema_for! (asset_bookmarks); serde_json ::
        to_string(& schema).unwrap()
    }
} impl asset_bookmarks
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        {
            fields.push(("asset_id", & self.asset_id as & dyn std :: fmt ::
            Debug));
        } if ! self.email.is_none()
        { fields.push(("email", & self.email as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        { fields.push("asset_id"); } if ! self.email.is_none()
        { fields.push("email"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.asset_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_id", count));
        } if ! self.email.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "email", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for asset_bookmarks
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "asset_bookmarks",
        "asset_id = $1 AND email = $2"); let sqlx_query : sqlx :: query ::
        Query < sqlx :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).bind(v.email.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for asset_bookmarks
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["asset_id", "email"]; println! ("Entering update");
        let active_fields : Vec < & str > = value.non_null_field_names(); let
        mut first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "asset_bookmarks")); if
        value.asset_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "asset_id")).push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.email.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "email")).push_bind(value.email.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "asset_id")).push_bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap());
        sqlx_query.push(format!
        (" and {} = ", "email")).push_bind(v.email.as_ref().unwrap()); ;
        println! ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for asset_bookmarks
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < asset_bookmarks > , sqlx :: Error > = sqlx
        :: query_as :: < _, asset_bookmarks >
        ("SELECT cast(asset_id as varchar) as asset_id,email FROM asset_bookmarks WHERE asset_id = $1 AND email = $2").bind(Uuid
        ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).bind(v.email.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for asset_bookmarks
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,email FROM asset_bookmarks".to_string());
        println!
        ("{}",
        "SELECT cast(asset_id as varchar) as asset_id,email FROM asset_bookmarks");
        let final_query : sqlx :: query :: QueryAs < _, asset_bookmarks, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for asset_bookmarks
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,email FROM asset_bookmarks".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.asset_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap());
        }; if value.email.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "email")); count += 1;
            sqlx_query.push_bind(value.email.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, asset_bookmarks, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for asset_bookmarks
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "asset_bookmarks",
        "asset_id,email", placeholders.join(", "),
        "cast(asset_id as text) as asset_id,email"); println! ("{}", query);
        let sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(value.asset_id.as_ref().unwrap()).unwrap()).bind(value.email.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/asset_bookmarks/{asset_id}/{email}")] async fn
get_asset_bookmarks_by_asset_id_and_email_handler(path : web :: Path <
asset_bookmarks > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < asset_bookmarks > , sqlx :: Error > = asset_bookmarks ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/asset_bookmarks/{asset_id}/{email}")] async fn
patch_asset_bookmarks_by_asset_id_and_email_handler(path : web :: Path <
asset_bookmarks > , json : web :: Json < asset_bookmarks > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = asset_bookmarks :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/asset_bookmarks")] async fn
get_asset_bookmarks_handler(info : web :: Query < asset_bookmarks > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < asset_bookmarks > ,
    sqlx :: Error > = asset_bookmarks :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/asset_bookmarks")] async fn
get_asset_bookmarks_handler_schema() -> impl Responder
{
    let b = schema_for! (asset_bookmarks); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/asset_bookmarks")] async fn
post_asset_bookmarks_by_asset_id_and_email_handler(json : web :: Json <
asset_bookmarks > , pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    asset_bookmarks :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = asset_bookmarks :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/asset_bookmarks/{asset_id}/{email}")] async fn
delete_asset_bookmarks_by_asset_id_and_email_handler(path : web :: Path <
asset_bookmarks > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res =
    asset_bookmarks :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct asset_collection
{ pub asset_id : Option < String > , pub collection_id : Option < String > , }
impl Schema for asset_collection
{
    fn schema() -> String
    {
        let schema = schema_for! (asset_collection); serde_json ::
        to_string(& schema).unwrap()
    }
} impl asset_collection
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        {
            fields.push(("asset_id", & self.asset_id as & dyn std :: fmt ::
            Debug));
        } if ! self.collection_id.is_none()
        {
            fields.push(("collection_id", & self.collection_id as & dyn std ::
            fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        { fields.push("asset_id"); } if ! self.collection_id.is_none()
        { fields.push("collection_id"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.asset_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_id", count));
        } if ! self.collection_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "collection_id", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for asset_collection
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "asset_collection",
        "asset_id = $1 AND collection_id = $2"); let sqlx_query : sqlx ::
        query :: Query < sqlx :: Postgres, sqlx :: postgres :: PgArguments > =
        sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).bind(Uuid ::
        parse_str(v.collection_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for asset_collection
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["asset_id", "collection_id"]; println!
        ("Entering update"); let active_fields : Vec < & str > =
        value.non_null_field_names(); let mut first = true; let mut sqlx_query
        = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "asset_collection")); if
        value.asset_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "asset_id")).push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.collection_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "collection_id")).push_bind(Uuid ::
            parse_str(value.collection_id.as_ref().unwrap()).unwrap()); first
            = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "asset_id")).push_bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap());
        sqlx_query.push(format!
        (" and {}  = ",
        "collection_id")).push_bind(Uuid ::
        parse_str(v.collection_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for asset_collection
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < asset_collection > , sqlx :: Error > = sqlx
        :: query_as :: < _, asset_collection >
        ("SELECT cast(asset_id as varchar) as asset_id,cast(collection_id as varchar) as collection_id FROM asset_collection WHERE asset_id = $1 AND collection_id = $2").bind(Uuid
        ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).bind(Uuid ::
        parse_str(v.collection_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for asset_collection
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,cast(collection_id as varchar) as collection_id FROM asset_collection".to_string());
        println!
        ("{}",
        "SELECT cast(asset_id as varchar) as asset_id,cast(collection_id as varchar) as collection_id FROM asset_collection");
        let final_query : sqlx :: query :: QueryAs < _, asset_collection, _ >
        = sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for asset_collection
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,cast(collection_id as varchar) as collection_id FROM asset_collection".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.asset_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap());
        }; if value.collection_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "collection_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.collection_id.as_ref().unwrap()).unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, asset_collection, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for asset_collection
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "asset_collection",
        "asset_id,collection_id", placeholders.join(", "),
        "cast(asset_id as text) as asset_id,cast(collection_id as text) as collection_id");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(value.asset_id.as_ref().unwrap()).unwrap()).bind(Uuid ::
        parse_str(value.collection_id.as_ref().unwrap()).unwrap());
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/asset_collection/{asset_id}/{collection_id}")] async fn
get_asset_collection_by_asset_id_and_collection_id_handler(path : web :: Path
< asset_collection > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < asset_collection > , sqlx :: Error > = asset_collection ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/asset_collection/{asset_id}/{collection_id}")] async fn
patch_asset_collection_by_asset_id_and_collection_id_handler(path : web ::
Path < asset_collection > , json : web :: Json < asset_collection > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = asset_collection :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/asset_collection")] async fn
get_asset_collection_handler(info : web :: Query < asset_collection > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < asset_collection > ,
    sqlx :: Error > = asset_collection :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/asset_collection")] async fn
get_asset_collection_handler_schema() -> impl Responder
{
    let b = schema_for! (asset_collection); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/asset_collection")] async fn
post_asset_collection_by_asset_id_and_collection_id_handler(json : web :: Json
< asset_collection > , pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    asset_collection :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = asset_collection :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/asset_collection/{asset_id}/{collection_id}")] async fn
delete_asset_collection_by_asset_id_and_collection_id_handler(path : web ::
Path < asset_collection > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res =
    asset_collection :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct asset_product
{ pub asset_id : Option < String > , pub product_id : Option < String > , }
impl Schema for asset_product
{
    fn schema() -> String
    {
        let schema = schema_for! (asset_product); serde_json ::
        to_string(& schema).unwrap()
    }
} impl asset_product
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        {
            fields.push(("asset_id", & self.asset_id as & dyn std :: fmt ::
            Debug));
        } if ! self.product_id.is_none()
        {
            fields.push(("product_id", & self.product_id as & dyn std :: fmt
            :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        { fields.push("asset_id"); } if ! self.product_id.is_none()
        { fields.push("product_id"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.asset_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_id", count));
        } if ! self.product_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "product_id", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for asset_product
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "asset_product",
        "asset_id = $1 AND product_id = $2"); let sqlx_query : sqlx :: query
        :: Query < sqlx :: Postgres, sqlx :: postgres :: PgArguments > = sqlx
        ::
        query(&
        query).bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).bind(v.product_id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for asset_product
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["asset_id", "product_id"]; println!
        ("Entering update"); let active_fields : Vec < & str > =
        value.non_null_field_names(); let mut first = true; let mut sqlx_query
        = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "asset_product")); if
        value.asset_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "asset_id")).push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.product_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "product_id")).push_bind(value.product_id.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "asset_id")).push_bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap());
        sqlx_query.push(format!
        (" and {} = ",
        "product_id")).push_bind(v.product_id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for asset_product
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < asset_product > , sqlx :: Error > = sqlx ::
        query_as :: < _, asset_product >
        ("SELECT cast(asset_id as varchar) as asset_id,product_id FROM asset_product WHERE asset_id = $1 AND product_id = $2").bind(Uuid
        ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).bind(v.product_id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for asset_product
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,product_id FROM asset_product".to_string());
        println!
        ("{}",
        "SELECT cast(asset_id as varchar) as asset_id,product_id FROM asset_product");
        let final_query : sqlx :: query :: QueryAs < _, asset_product, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for asset_product
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,product_id FROM asset_product".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.asset_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap());
        }; if value.product_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "product_id")); count += 1;
            sqlx_query.push_bind(value.product_id.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, asset_product, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for asset_product
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "asset_product",
        "asset_id,product_id", placeholders.join(", "),
        "cast(asset_id as text) as asset_id,product_id"); println!
        ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx ::
        Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(value.asset_id.as_ref().unwrap()).unwrap()).bind(value.product_id.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/asset_product/{asset_id}/{product_id}")] async fn
get_asset_product_by_asset_id_and_product_id_handler(path : web :: Path <
asset_product > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < asset_product > , sqlx :: Error > = asset_product ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/asset_product/{asset_id}/{product_id}")] async fn
patch_asset_product_by_asset_id_and_product_id_handler(path : web :: Path <
asset_product > , json : web :: Json < asset_product > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = asset_product :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/asset_product")] async fn
get_asset_product_handler(info : web :: Query < asset_product > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < asset_product > , sqlx
    :: Error > = asset_product :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/asset_product")] async fn
get_asset_product_handler_schema() -> impl Responder
{
    let b = schema_for! (asset_product); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/asset_product")] async fn
post_asset_product_by_asset_id_and_product_id_handler(json : web :: Json <
asset_product > , pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    asset_product :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = asset_product :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/asset_product/{asset_id}/{product_id}")] async fn
delete_asset_product_by_asset_id_and_product_id_handler(path : web :: Path <
asset_product > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = asset_product
    :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct asset_ratings
{
    pub rating_id : Option < String > , pub rating_value : Option < f64 > ,
    pub createdby : Option < String > , pub related_asset : Option < String >
    ,
} impl Schema for asset_ratings
{
    fn schema() -> String
    {
        let schema = schema_for! (asset_ratings); serde_json ::
        to_string(& schema).unwrap()
    }
} impl asset_ratings
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.rating_id.is_none()
        {
            fields.push(("rating_id", & self.rating_id as & dyn std :: fmt ::
            Debug));
        } if ! self.rating_value.is_none()
        {
            fields.push(("rating_value", & self.rating_value as & dyn std ::
            fmt :: Debug));
        } if ! self.createdby.is_none()
        {
            fields.push(("createdby", & self.createdby as & dyn std :: fmt ::
            Debug));
        } if ! self.related_asset.is_none()
        {
            fields.push(("related_asset", & self.related_asset as & dyn std ::
            fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.rating_id.is_none()
        { fields.push("rating_id"); } if ! self.rating_value.is_none()
        { fields.push("rating_value"); } if ! self.createdby.is_none()
        { fields.push("createdby"); } if ! self.related_asset.is_none()
        { fields.push("related_asset"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.rating_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "rating_id", count));
        } if ! self.rating_value.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "rating_value", count));
        } if ! self.createdby.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "createdby", count));
        } if ! self.related_asset.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "related_asset", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for asset_ratings
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "asset_ratings", "rating_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.rating_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for asset_ratings
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["rating_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "asset_ratings")); if
        value.rating_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "rating_id")).push_bind(Uuid ::
            parse_str(value.rating_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.rating_value.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "rating_value")).push_bind(value.rating_value.as_ref().unwrap());
            first = false;
        }; if value.createdby.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "createdby")).push_bind(value.createdby.as_ref().unwrap()); first
            = false;
        }; if value.related_asset.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "related_asset")).push_bind(Uuid ::
            parse_str(value.related_asset.as_ref().unwrap()).unwrap()); first
            = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "rating_id")).push_bind(Uuid ::
        parse_str(v.rating_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for asset_ratings
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < asset_ratings > , sqlx :: Error > = sqlx ::
        query_as :: < _, asset_ratings >
        ("SELECT cast(rating_id as varchar) as rating_id,rating_value,createdby,cast(related_asset as varchar) as related_asset FROM asset_ratings WHERE rating_id = $1").bind(Uuid
        ::
        parse_str(v.rating_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for asset_ratings
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(rating_id as varchar) as rating_id,rating_value,createdby,cast(related_asset as varchar) as related_asset FROM asset_ratings".to_string());
        println!
        ("{}",
        "SELECT cast(rating_id as varchar) as rating_id,rating_value,createdby,cast(related_asset as varchar) as related_asset FROM asset_ratings");
        let final_query : sqlx :: query :: QueryAs < _, asset_ratings, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for asset_ratings
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(rating_id as varchar) as rating_id,rating_value,createdby,cast(related_asset as varchar) as related_asset FROM asset_ratings".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.rating_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "rating_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.rating_id.as_ref().unwrap()).unwrap());
        }; if value.rating_value.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "rating_value")); count += 1;
            sqlx_query.push_bind(value.rating_value.as_ref().unwrap());
        }; if value.createdby.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "createdby")); count += 1;
            sqlx_query.push_bind(value.createdby.as_ref().unwrap());
        }; if value.related_asset.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "related_asset")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.related_asset.as_ref().unwrap()).unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, asset_ratings, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for asset_ratings
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 3usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "asset_ratings",
        "rating_value,createdby,related_asset", placeholders.join(", "),
        "cast(rating_id as text) as rating_id,rating_value,createdby,cast(related_asset as text) as related_asset");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.rating_value.as_ref().or(None)).bind(value.createdby.as_ref().or(None)).bind(Uuid
        :: parse_str(value.related_asset.as_ref().unwrap()).unwrap());
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/asset_ratings/{rating_id}")] async fn
get_asset_ratings_by_rating_id_handler(path : web :: Path < asset_ratings > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < asset_ratings > , sqlx :: Error > = asset_ratings ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/asset_ratings/{rating_id}")] async fn
patch_asset_ratings_by_rating_id_handler(path : web :: Path < asset_ratings >
, json : web :: Json < asset_ratings > , pool : web :: Data < PgPool >) ->
impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = asset_ratings :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/asset_ratings")] async fn
get_asset_ratings_handler(info : web :: Query < asset_ratings > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < asset_ratings > , sqlx
    :: Error > = asset_ratings :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/asset_ratings")] async fn
get_asset_ratings_handler_schema() -> impl Responder
{
    let b = schema_for! (asset_ratings); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/asset_ratings")] async fn
post_asset_ratings_by_rating_id_handler(json : web :: Json < asset_ratings > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    asset_ratings :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = asset_ratings :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/asset_ratings/{rating_id}")] async fn
delete_asset_ratings_by_rating_id_handler(path : web :: Path < asset_ratings >
, pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = asset_ratings
    :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct asset_types
{ pub type_id : Option < String > , pub type_name : Option < String > , } impl
Schema for asset_types
{
    fn schema() -> String
    {
        let schema = schema_for! (asset_types); serde_json ::
        to_string(& schema).unwrap()
    }
} impl asset_types
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.type_id.is_none()
        {
            fields.push(("type_id", & self.type_id as & dyn std :: fmt ::
            Debug));
        } if ! self.type_name.is_none()
        {
            fields.push(("type_name", & self.type_name as & dyn std :: fmt ::
            Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.type_id.is_none()
        { fields.push("type_id"); } if ! self.type_name.is_none()
        { fields.push("type_name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.type_id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "type_id", count)); }
        if ! self.type_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "type_name", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for asset_types
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "asset_types", "type_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(v.type_id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for asset_types
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["type_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "asset_types")); if
        value.type_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "type_id")).push_bind(value.type_id.as_ref().unwrap()); first =
            false;
        }; if value.type_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "type_name")).push_bind(value.type_name.as_ref().unwrap()); first
            = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "type_id")).push_bind(v.type_id.as_ref().unwrap()); ;
        println! ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for asset_types
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < asset_types > , sqlx :: Error > = sqlx ::
        query_as :: < _, asset_types >
        ("SELECT type_id,type_name FROM asset_types WHERE type_id = $1").bind(v.type_id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for asset_types
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT type_id,type_name FROM asset_types".to_string()); println!
        ("{}", "SELECT type_id,type_name FROM asset_types"); let final_query :
        sqlx :: query :: QueryAs < _, asset_types, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for asset_types
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT type_id,type_name FROM asset_types".to_string()); let mut
        count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.type_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "type_id")); count += 1;
            sqlx_query.push_bind(value.type_id.as_ref().unwrap());
        }; if value.type_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "type_name")); count += 1;
            sqlx_query.push_bind(value.type_name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, asset_types, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for asset_types
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "asset_types",
        "type_id,type_name", placeholders.join(", "), "type_id,type_name");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.type_id.as_ref().or(None)).bind(value.type_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/asset_types/{type_id}")] async fn
get_asset_types_by_type_id_handler(path : web :: Path < asset_types > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < asset_types > , sqlx :: Error > = asset_types ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/asset_types/{type_id}")] async fn
patch_asset_types_by_type_id_handler(path : web :: Path < asset_types > , json
: web :: Json < asset_types > , pool : web :: Data < PgPool >) -> impl
Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = asset_types :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/asset_types")] async fn
get_asset_types_handler(info : web :: Query < asset_types > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < asset_types > , sqlx
    :: Error > = asset_types :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/asset_types")] async fn get_asset_types_handler_schema()
-> impl Responder
{
    let b = schema_for! (asset_types); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/asset_types")] async fn
post_asset_types_by_type_id_handler(json : web :: Json < asset_types > , pool
: web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    asset_types :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = asset_types :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/asset_types/{type_id}")] async fn
delete_asset_types_by_type_id_handler(path : web :: Path < asset_types > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = asset_types
    :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct assets
{
    pub asset_id : Option < String > , pub asset_name : Option < String > ,
    pub asset_owner : Option < String > , pub asset_description : Option <
    String > , pub asset_type : Option < String > , pub asset_link : Option <
    String > , pub created_at : Option < String > , pub updated_at : Option <
    String > , pub asset_offering_type : Option < String > , pub asset_brand :
    Option < String > , pub asset_practice : Option < String > , pub
    is_ip_cleared : Option < bool > , pub is_sellable : Option < bool > , pub
    asset_rating_avg : Option < f64 > , pub asset_collaborators : Option < Vec
    < String > > , pub asset_owner_name : Option < String > , pub asset_geo :
    Option < String > , pub asset_market : Option < String > ,
} impl Schema for assets
{
    fn schema() -> String
    {
        let schema = schema_for! (assets); serde_json ::
        to_string(& schema).unwrap()
    }
} impl assets
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        {
            fields.push(("asset_id", & self.asset_id as & dyn std :: fmt ::
            Debug));
        } if ! self.asset_name.is_none()
        {
            fields.push(("asset_name", & self.asset_name as & dyn std :: fmt
            :: Debug));
        } if ! self.asset_owner.is_none()
        {
            fields.push(("asset_owner", & self.asset_owner as & dyn std :: fmt
            :: Debug));
        } if ! self.asset_description.is_none()
        {
            fields.push(("asset_description", & self.asset_description as &
            dyn std :: fmt :: Debug));
        } if ! self.asset_type.is_none()
        {
            fields.push(("asset_type", & self.asset_type as & dyn std :: fmt
            :: Debug));
        } if ! self.asset_link.is_none()
        {
            fields.push(("asset_link", & self.asset_link as & dyn std :: fmt
            :: Debug));
        } if ! self.created_at.is_none()
        {
            fields.push(("created_at", & self.created_at as & dyn std :: fmt
            :: Debug));
        } if ! self.updated_at.is_none()
        {
            fields.push(("updated_at", & self.updated_at as & dyn std :: fmt
            :: Debug));
        } if ! self.asset_offering_type.is_none()
        {
            fields.push(("asset_offering_type", & self.asset_offering_type as
            & dyn std :: fmt :: Debug));
        } if ! self.asset_brand.is_none()
        {
            fields.push(("asset_brand", & self.asset_brand as & dyn std :: fmt
            :: Debug));
        } if ! self.asset_practice.is_none()
        {
            fields.push(("asset_practice", & self.asset_practice as & dyn std
            :: fmt :: Debug));
        } if ! self.is_ip_cleared.is_none()
        {
            fields.push(("is_ip_cleared", & self.is_ip_cleared as & dyn std ::
            fmt :: Debug));
        } if ! self.is_sellable.is_none()
        {
            fields.push(("is_sellable", & self.is_sellable as & dyn std :: fmt
            :: Debug));
        } if ! self.asset_rating_avg.is_none()
        {
            fields.push(("asset_rating_avg", & self.asset_rating_avg as & dyn
            std :: fmt :: Debug));
        } if ! self.asset_collaborators.is_none()
        {
            fields.push(("asset_collaborators", & self.asset_collaborators as
            & dyn std :: fmt :: Debug));
        } if ! self.asset_owner_name.is_none()
        {
            fields.push(("asset_owner_name", & self.asset_owner_name as & dyn
            std :: fmt :: Debug));
        } if ! self.asset_geo.is_none()
        {
            fields.push(("asset_geo", & self.asset_geo as & dyn std :: fmt ::
            Debug));
        } if ! self.asset_market.is_none()
        {
            fields.push(("asset_market", & self.asset_market as & dyn std ::
            fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.asset_id.is_none()
        { fields.push("asset_id"); } if ! self.asset_name.is_none()
        { fields.push("asset_name"); } if ! self.asset_owner.is_none()
        { fields.push("asset_owner"); } if ! self.asset_description.is_none()
        { fields.push("asset_description"); } if ! self.asset_type.is_none()
        { fields.push("asset_type"); } if ! self.asset_link.is_none()
        { fields.push("asset_link"); } if ! self.created_at.is_none()
        { fields.push("created_at"); } if ! self.updated_at.is_none()
        { fields.push("updated_at"); } if ! self.asset_offering_type.is_none()
        { fields.push("asset_offering_type"); } if !
        self.asset_brand.is_none() { fields.push("asset_brand"); } if !
        self.asset_practice.is_none() { fields.push("asset_practice"); } if !
        self.is_ip_cleared.is_none() { fields.push("is_ip_cleared"); } if !
        self.is_sellable.is_none() { fields.push("is_sellable"); } if !
        self.asset_rating_avg.is_none() { fields.push("asset_rating_avg"); }
        if ! self.asset_collaborators.is_none()
        { fields.push("asset_collaborators"); } if !
        self.asset_owner_name.is_none() { fields.push("asset_owner_name"); }
        if ! self.asset_geo.is_none() { fields.push("asset_geo"); } if !
        self.asset_market.is_none() { fields.push("asset_market"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.asset_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_id", count));
        } if ! self.asset_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_name", count));
        } if ! self.asset_owner.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_owner", count));
        } if ! self.asset_description.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_description", count));
        } if ! self.asset_type.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_type", count));
        } if ! self.asset_link.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_link", count));
        } if ! self.created_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "created_at", count));
        } if ! self.updated_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "updated_at", count));
        } if ! self.asset_offering_type.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_offering_type", count));
        } if ! self.asset_brand.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_brand", count));
        } if ! self.asset_practice.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_practice", count));
        } if ! self.is_ip_cleared.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "is_ip_cleared", count));
        } if ! self.is_sellable.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "is_sellable", count));
        } if ! self.asset_rating_avg.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_rating_avg", count));
        } if ! self.asset_owner_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_owner_name", count));
        } if ! self.asset_geo.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_geo", count));
        } if ! self.asset_market.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_market", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for assets
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "assets", "asset_id = $1"); let sqlx_query
        : sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx ::
        query(&
        query).bind(Uuid :: parse_str(v.asset_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for assets
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["asset_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "assets")); if value.asset_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "asset_id")).push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.asset_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_name")).push_bind(value.asset_name.as_ref().unwrap());
            first = false;
        }; if value.asset_owner.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_owner")).push_bind(value.asset_owner.as_ref().unwrap());
            first = false;
        }; if value.asset_description.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_description")).push_bind(value.asset_description.as_ref().unwrap());
            first = false;
        }; if value.asset_type.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_type")).push_bind(value.asset_type.as_ref().unwrap());
            first = false;
        }; if value.asset_link.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_link")).push_bind(value.asset_link.as_ref().unwrap());
            first = false;
        }; if value.created_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "created_at")).push_bind(value.created_at.as_ref().unwrap());
            first = false;
        }; if value.updated_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "updated_at")).push_bind(value.updated_at.as_ref().unwrap());
            first = false;
        }; if value.asset_offering_type.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_offering_type")).push_bind(value.asset_offering_type.as_ref().unwrap());
            first = false;
        }; if value.asset_brand.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_brand")).push_bind(value.asset_brand.as_ref().unwrap());
            first = false;
        }; if value.asset_practice.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_practice")).push_bind(value.asset_practice.as_ref().unwrap());
            first = false;
        }; if value.is_ip_cleared.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "is_ip_cleared")).push_bind(value.is_ip_cleared.as_ref().unwrap());
            first = false;
        }; if value.is_sellable.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "is_sellable")).push_bind(value.is_sellable.as_ref().unwrap());
            first = false;
        }; if value.asset_rating_avg.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_rating_avg")).push_bind(value.asset_rating_avg.as_ref().unwrap());
            first = false;
        }; if value.asset_collaborators.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_collaborators")).push_bind(value.asset_collaborators.as_ref().unwrap());
            first = false;
        }; if value.asset_owner_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_owner_name")).push_bind(value.asset_owner_name.as_ref().unwrap());
            first = false;
        }; if value.asset_geo.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_geo")).push_bind(value.asset_geo.as_ref().unwrap()); first
            = false;
        }; if value.asset_market.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "asset_market")).push_bind(value.asset_market.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "asset_id")).push_bind(Uuid ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for assets
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < assets > , sqlx :: Error > = sqlx ::
        query_as :: < _, assets >
        ("SELECT cast(asset_id as varchar) as asset_id,asset_name,asset_owner,asset_description,asset_type,asset_link,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,asset_offering_type,asset_brand,asset_practice,is_ip_cleared,is_sellable,asset_rating_avg,asset_collaborators,asset_owner_name,asset_geo,asset_market FROM assets WHERE asset_id = $1").bind(Uuid
        ::
        parse_str(v.asset_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for assets
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,asset_name,asset_owner,asset_description,asset_type,asset_link,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,asset_offering_type,asset_brand,asset_practice,is_ip_cleared,is_sellable,asset_rating_avg,asset_collaborators,asset_owner_name,asset_geo,asset_market FROM assets".to_string());
        println!
        ("{}",
        "SELECT cast(asset_id as varchar) as asset_id,asset_name,asset_owner,asset_description,asset_type,asset_link,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,asset_offering_type,asset_brand,asset_practice,is_ip_cleared,is_sellable,asset_rating_avg,asset_collaborators,asset_owner_name,asset_geo,asset_market FROM assets");
        let final_query : sqlx :: query :: QueryAs < _, assets, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for assets
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(asset_id as varchar) as asset_id,asset_name,asset_owner,asset_description,asset_type,asset_link,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,asset_offering_type,asset_brand,asset_practice,is_ip_cleared,is_sellable,asset_rating_avg,asset_collaborators,asset_owner_name,asset_geo,asset_market FROM assets".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.asset_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap());
        }; if value.asset_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_name")); count += 1;
            sqlx_query.push_bind(value.asset_name.as_ref().unwrap());
        }; if value.asset_owner.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_owner")); count += 1;
            sqlx_query.push_bind(value.asset_owner.as_ref().unwrap());
        }; if value.asset_description.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_description")); count +=
            1;
            sqlx_query.push_bind(value.asset_description.as_ref().unwrap());
        }; if value.asset_type.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_type")); count += 1;
            sqlx_query.push_bind(value.asset_type.as_ref().unwrap());
        }; if value.asset_link.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_link")); count += 1;
            sqlx_query.push_bind(value.asset_link.as_ref().unwrap());
        }; if value.created_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "created_at")); count += 1;
            sqlx_query.push_bind(value.created_at.as_ref().unwrap());
        }; if value.updated_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "updated_at")); count += 1;
            sqlx_query.push_bind(value.updated_at.as_ref().unwrap());
        }; if value.asset_offering_type.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_offering_type")); count
            += 1;
            sqlx_query.push_bind(value.asset_offering_type.as_ref().unwrap());
        }; if value.asset_brand.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_brand")); count += 1;
            sqlx_query.push_bind(value.asset_brand.as_ref().unwrap());
        }; if value.asset_practice.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_practice")); count += 1;
            sqlx_query.push_bind(value.asset_practice.as_ref().unwrap());
        }; if value.is_ip_cleared.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "is_ip_cleared")); count += 1;
            sqlx_query.push_bind(value.is_ip_cleared.as_ref().unwrap());
        }; if value.is_sellable.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "is_sellable")); count += 1;
            sqlx_query.push_bind(value.is_sellable.as_ref().unwrap());
        }; if value.asset_rating_avg.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_rating_avg")); count +=
            1; sqlx_query.push_bind(value.asset_rating_avg.as_ref().unwrap());
        }; if value.asset_owner_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_owner_name")); count +=
            1; sqlx_query.push_bind(value.asset_owner_name.as_ref().unwrap());
        }; if value.asset_geo.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_geo")); count += 1;
            sqlx_query.push_bind(value.asset_geo.as_ref().unwrap());
        }; if value.asset_market.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_market")); count += 1;
            sqlx_query.push_bind(value.asset_market.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, assets, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for assets
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 12usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "assets",
        "asset_name,asset_owner,asset_description,asset_type,asset_link,asset_offering_type,asset_brand,asset_practice,asset_collaborators,asset_owner_name,asset_geo,asset_market",
        placeholders.join(", "),
        "cast(asset_id as text) as asset_id,asset_name,asset_owner,asset_description,asset_type,asset_link,cast(created_at as text) as created_at,cast(updated_at as text) as updated_at,asset_offering_type,asset_brand,asset_practice,is_ip_cleared,is_sellable,asset_rating_avg,asset_collaborators,asset_owner_name,asset_geo,asset_market");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.asset_name.as_ref().or(None)).bind(value.asset_owner.as_ref().or(None)).bind(value.asset_description.as_ref().or(None)).bind(value.asset_type.as_ref().or(None)).bind(value.asset_link.as_ref().or(None)).bind(value.asset_offering_type.as_ref().or(None)).bind(value.asset_brand.as_ref().or(None)).bind(value.asset_practice.as_ref().or(None)).bind(value.asset_collaborators.as_ref().or(None)).bind(value.asset_owner_name.as_ref().or(None)).bind(value.asset_geo.as_ref().or(None)).bind(value.asset_market.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/assets/{asset_id}")] async fn
get_assets_by_asset_id_handler(path : web :: Path < assets > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < assets > , sqlx :: Error > = assets :: select_where(p, v).await; match
    res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/assets/{asset_id}")] async fn
patch_assets_by_asset_id_handler(path : web :: Path < assets > , json : web ::
Json < assets > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = assets :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/assets")] async fn
get_assets_handler(info : web :: Query < assets > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < assets > , sqlx ::
    Error > = assets :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/assets")] async fn get_assets_handler_schema() -> impl
Responder
{
    let b = schema_for! (assets); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/assets")] async fn
post_assets_by_asset_id_handler(json : web :: Json < assets > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = assets ::
    insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = assets :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/assets/{asset_id}")] async fn
delete_assets_by_asset_id_handler(path : web :: Path < assets > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = assets ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct attributes
{ pub id : Option < i32 > , pub name : Option < String > , } impl Schema for
attributes
{
    fn schema() -> String
    {
        let schema = schema_for! (attributes); serde_json ::
        to_string(& schema).unwrap()
    }
} impl attributes
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push(("id", & self.id as & dyn std :: fmt :: Debug)); } if !
        self.name.is_none()
        { fields.push(("name", & self.name as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push("id"); } if ! self.name.is_none()
        { fields.push("name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "id", count)); } if !
        self.name.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "name", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for attributes
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "attributes", "id = $1"); let sqlx_query :
        sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx :: query(& query).bind(v.id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for attributes
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "attributes")); if value.id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "id")).push_bind(value.id.as_ref().unwrap()); first =
            false;
        }; if value.name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "name")).push_bind(value.name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "id")).push_bind(v.id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for attributes
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < attributes > , sqlx :: Error > = sqlx ::
        query_as :: < _, attributes >
        ("SELECT id,name FROM attributes WHERE id = $1").bind(v.id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for attributes
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,name FROM attributes".to_string()); println!
        ("{}", "SELECT id,name FROM attributes"); let final_query : sqlx ::
        query :: QueryAs < _, attributes, _ > = sqlx_query.build_query_as();
        final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for attributes
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,name FROM attributes".to_string()); let mut count = 0;
        if value.where_clause().len() > 0 { sqlx_query.push(" where "); } if
        value.id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "id")); count += 1;
            sqlx_query.push_bind(value.id.as_ref().unwrap());
        }; if value.name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "name")); count += 1;
            sqlx_query.push_bind(value.name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, attributes, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for attributes
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 1usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "attributes", "name",
        placeholders.join(", "), "id,name"); println! ("{}", query); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(value.name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/attributes/{id}")] async fn
get_attributes_by_id_handler(path : web :: Path < attributes > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < attributes > , sqlx :: Error > = attributes :: select_where(p, v).await;
    match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/attributes/{id}")] async fn
patch_attributes_by_id_handler(path : web :: Path < attributes > , json : web
:: Json < attributes > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = attributes :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/attributes")] async fn
get_attributes_handler(info : web :: Query < attributes > , pool : web :: Data
< PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < attributes > , sqlx ::
    Error > = attributes :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/attributes")] async fn get_attributes_handler_schema()
-> impl Responder
{
    let b = schema_for! (attributes); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/attributes")] async fn
post_attributes_by_id_handler(json : web :: Json < attributes > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = attributes
    :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = attributes :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/attributes/{id}")] async fn
delete_attributes_by_id_handler(path : web :: Path < attributes > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = attributes ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct attributevalues
{
    pub id : Option < i32 > , pub aid : Option < i32 > , pub value : Option <
    String > ,
} impl Schema for attributevalues
{
    fn schema() -> String
    {
        let schema = schema_for! (attributevalues); serde_json ::
        to_string(& schema).unwrap()
    }
} impl attributevalues
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push(("id", & self.id as & dyn std :: fmt :: Debug)); } if !
        self.aid.is_none()
        { fields.push(("aid", & self.aid as & dyn std :: fmt :: Debug)); } if
        ! self.value.is_none()
        { fields.push(("value", & self.value as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push("id"); } if ! self.aid.is_none() { fields.push("aid"); }
        if ! self.value.is_none() { fields.push("value"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "id", count)); } if !
        self.aid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "aid", count)); } if
        ! self.value.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "value", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for attributevalues
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "attributevalues", "id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(v.id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for attributevalues
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "attributevalues")); if
        value.id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "id")).push_bind(value.id.as_ref().unwrap()); first =
            false;
        }; if value.aid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "aid")).push_bind(value.aid.as_ref().unwrap()); first
            = false;
        }; if value.value.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "value")).push_bind(value.value.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "id")).push_bind(v.id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for attributevalues
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < attributevalues > , sqlx :: Error > = sqlx
        :: query_as :: < _, attributevalues >
        ("SELECT id,aid,value FROM attributevalues WHERE id = $1").bind(v.id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for attributevalues
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,aid,value FROM attributevalues".to_string()); println!
        ("{}", "SELECT id,aid,value FROM attributevalues"); let final_query :
        sqlx :: query :: QueryAs < _, attributevalues, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for attributevalues
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,aid,value FROM attributevalues".to_string()); let mut
        count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "id")); count += 1;
            sqlx_query.push_bind(value.id.as_ref().unwrap());
        }; if value.aid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "aid")); count += 1;
            sqlx_query.push_bind(value.aid.as_ref().unwrap());
        }; if value.value.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "value")); count += 1;
            sqlx_query.push_bind(value.value.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, attributevalues, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for attributevalues
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "attributevalues",
        "aid,value", placeholders.join(", "), "id,aid,value"); println!
        ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx ::
        Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.aid.as_ref().or(None)).bind(value.value.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/attributevalues/{id}")] async fn
get_attributevalues_by_id_handler(path : web :: Path < attributevalues > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < attributevalues > , sqlx :: Error > = attributevalues ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/attributevalues/{id}")] async fn
patch_attributevalues_by_id_handler(path : web :: Path < attributevalues > ,
json : web :: Json < attributevalues > , pool : web :: Data < PgPool >) ->
impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = attributevalues :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/attributevalues")] async fn
get_attributevalues_handler(info : web :: Query < attributevalues > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < attributevalues > ,
    sqlx :: Error > = attributevalues :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/attributevalues")] async fn
get_attributevalues_handler_schema() -> impl Responder
{
    let b = schema_for! (attributevalues); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/attributevalues")] async fn
post_attributevalues_by_id_handler(json : web :: Json < attributevalues > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    attributevalues :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = attributevalues :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/attributevalues/{id}")] async fn
delete_attributevalues_by_id_handler(path : web :: Path < attributevalues > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res =
    attributevalues :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct brands
{ pub brand_id : Option < String > , pub brand_name : Option < String > , }
impl Schema for brands
{
    fn schema() -> String
    {
        let schema = schema_for! (brands); serde_json ::
        to_string(& schema).unwrap()
    }
} impl brands
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.brand_id.is_none()
        {
            fields.push(("brand_id", & self.brand_id as & dyn std :: fmt ::
            Debug));
        } if ! self.brand_name.is_none()
        {
            fields.push(("brand_name", & self.brand_name as & dyn std :: fmt
            :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.brand_id.is_none()
        { fields.push("brand_id"); } if ! self.brand_name.is_none()
        { fields.push("brand_name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.brand_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "brand_id", count));
        } if ! self.brand_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "brand_name", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for brands
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "brands", "brand_id = $1"); let sqlx_query
        : sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx ::
        query(& query).bind(v.brand_id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for brands
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["brand_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "brands")); if value.brand_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "brand_id")).push_bind(value.brand_id.as_ref().unwrap()); first =
            false;
        }; if value.brand_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "brand_name")).push_bind(value.brand_name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "brand_id")).push_bind(v.brand_id.as_ref().unwrap());
        ; println! ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for brands
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < brands > , sqlx :: Error > = sqlx ::
        query_as :: < _, brands >
        ("SELECT brand_id,brand_name FROM brands WHERE brand_id = $1").bind(v.brand_id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for brands
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT brand_id,brand_name FROM brands".to_string()); println!
        ("{}", "SELECT brand_id,brand_name FROM brands"); let final_query :
        sqlx :: query :: QueryAs < _, brands, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for brands
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT brand_id,brand_name FROM brands".to_string()); let mut
        count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.brand_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "brand_id")); count += 1;
            sqlx_query.push_bind(value.brand_id.as_ref().unwrap());
        }; if value.brand_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "brand_name")); count += 1;
            sqlx_query.push_bind(value.brand_name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, brands, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for brands
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "brands",
        "brand_id,brand_name", placeholders.join(", "),
        "brand_id,brand_name"); println! ("{}", query); let sqlx_query : sqlx
        :: query :: Query < sqlx :: Postgres, sqlx :: postgres :: PgArguments
        > = sqlx ::
        query(&
        query).bind(value.brand_id.as_ref().or(None)).bind(value.brand_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/brands/{brand_id}")] async fn
get_brands_by_brand_id_handler(path : web :: Path < brands > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < brands > , sqlx :: Error > = brands :: select_where(p, v).await; match
    res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/brands/{brand_id}")] async fn
patch_brands_by_brand_id_handler(path : web :: Path < brands > , json : web ::
Json < brands > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = brands :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/brands")] async fn
get_brands_handler(info : web :: Query < brands > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < brands > , sqlx ::
    Error > = brands :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/brands")] async fn get_brands_handler_schema() -> impl
Responder
{
    let b = schema_for! (brands); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/brands")] async fn
post_brands_by_brand_id_handler(json : web :: Json < brands > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = brands ::
    insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = brands :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/brands/{brand_id}")] async fn
delete_brands_by_brand_id_handler(path : web :: Path < brands > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = brands ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct collections
{
    pub collection_id : Option < String > , pub collection_name : Option <
    String > , pub created_at : Option < String > , pub updated_at : Option <
    String > , pub collection_description : Option < String > , pub
    collection_owner : Option < String > , pub collection_collaborators :
    Option < Vec < String > > , pub collection_owner_name : Option < String >
    ,
} impl Schema for collections
{
    fn schema() -> String
    {
        let schema = schema_for! (collections); serde_json ::
        to_string(& schema).unwrap()
    }
} impl collections
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.collection_id.is_none()
        {
            fields.push(("collection_id", & self.collection_id as & dyn std ::
            fmt :: Debug));
        } if ! self.collection_name.is_none()
        {
            fields.push(("collection_name", & self.collection_name as & dyn
            std :: fmt :: Debug));
        } if ! self.created_at.is_none()
        {
            fields.push(("created_at", & self.created_at as & dyn std :: fmt
            :: Debug));
        } if ! self.updated_at.is_none()
        {
            fields.push(("updated_at", & self.updated_at as & dyn std :: fmt
            :: Debug));
        } if ! self.collection_description.is_none()
        {
            fields.push(("collection_description", &
            self.collection_description as & dyn std :: fmt :: Debug));
        } if ! self.collection_owner.is_none()
        {
            fields.push(("collection_owner", & self.collection_owner as & dyn
            std :: fmt :: Debug));
        } if ! self.collection_collaborators.is_none()
        {
            fields.push(("collection_collaborators", &
            self.collection_collaborators as & dyn std :: fmt :: Debug));
        } if ! self.collection_owner_name.is_none()
        {
            fields.push(("collection_owner_name", & self.collection_owner_name
            as & dyn std :: fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.collection_id.is_none()
        { fields.push("collection_id"); } if ! self.collection_name.is_none()
        { fields.push("collection_name"); } if ! self.created_at.is_none()
        { fields.push("created_at"); } if ! self.updated_at.is_none()
        { fields.push("updated_at"); } if !
        self.collection_description.is_none()
        { fields.push("collection_description"); } if !
        self.collection_owner.is_none() { fields.push("collection_owner"); }
        if ! self.collection_collaborators.is_none()
        { fields.push("collection_collaborators"); } if !
        self.collection_owner_name.is_none()
        { fields.push("collection_owner_name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.collection_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "collection_id", count));
        } if ! self.collection_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "collection_name", count));
        } if ! self.created_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "created_at", count));
        } if ! self.updated_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "updated_at", count));
        } if ! self.collection_description.is_none()
        {
            count += 1;
            fields.push(format!
            (" {} = ${} ", "collection_description", count));
        } if ! self.collection_owner.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "collection_owner", count));
        } if ! self.collection_owner_name.is_none()
        {
            count += 1;
            fields.push(format!
            (" {} = ${} ", "collection_owner_name", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for collections
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "collections", "collection_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.collection_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for collections
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["collection_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "collections")); if
        value.collection_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "collection_id")).push_bind(Uuid ::
            parse_str(value.collection_id.as_ref().unwrap()).unwrap()); first
            = false;
        }; if value.collection_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "collection_name")).push_bind(value.collection_name.as_ref().unwrap());
            first = false;
        }; if value.created_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "created_at")).push_bind(value.created_at.as_ref().unwrap());
            first = false;
        }; if value.updated_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "updated_at")).push_bind(value.updated_at.as_ref().unwrap());
            first = false;
        }; if value.collection_description.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "collection_description")).push_bind(value.collection_description.as_ref().unwrap());
            first = false;
        }; if value.collection_owner.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "collection_owner")).push_bind(value.collection_owner.as_ref().unwrap());
            first = false;
        }; if value.collection_collaborators.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "collection_collaborators")).push_bind(value.collection_collaborators.as_ref().unwrap());
            first = false;
        }; if value.collection_owner_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "collection_owner_name")).push_bind(value.collection_owner_name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "collection_id")).push_bind(Uuid ::
        parse_str(v.collection_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for collections
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < collections > , sqlx :: Error > = sqlx ::
        query_as :: < _, collections >
        ("SELECT cast(collection_id as varchar) as collection_id,collection_name,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,collection_description,collection_owner,collection_collaborators,collection_owner_name FROM collections WHERE collection_id = $1").bind(Uuid
        ::
        parse_str(v.collection_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for collections
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(collection_id as varchar) as collection_id,collection_name,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,collection_description,collection_owner,collection_collaborators,collection_owner_name FROM collections".to_string());
        println!
        ("{}",
        "SELECT cast(collection_id as varchar) as collection_id,collection_name,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,collection_description,collection_owner,collection_collaborators,collection_owner_name FROM collections");
        let final_query : sqlx :: query :: QueryAs < _, collections, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for collections
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(collection_id as varchar) as collection_id,collection_name,cast(created_at as varchar) as created_at,cast(updated_at as varchar) as updated_at,collection_description,collection_owner,collection_collaborators,collection_owner_name FROM collections".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.collection_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "collection_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.collection_id.as_ref().unwrap()).unwrap());
        }; if value.collection_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "collection_name")); count +=
            1; sqlx_query.push_bind(value.collection_name.as_ref().unwrap());
        }; if value.created_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "created_at")); count += 1;
            sqlx_query.push_bind(value.created_at.as_ref().unwrap());
        }; if value.updated_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "updated_at")); count += 1;
            sqlx_query.push_bind(value.updated_at.as_ref().unwrap());
        }; if value.collection_description.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "collection_description"));
            count += 1;
            sqlx_query.push_bind(value.collection_description.as_ref().unwrap());
        }; if value.collection_owner.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "collection_owner")); count +=
            1; sqlx_query.push_bind(value.collection_owner.as_ref().unwrap());
        }; if value.collection_owner_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "collection_owner_name"));
            count += 1;
            sqlx_query.push_bind(value.collection_owner_name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, collections, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for collections
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 5usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "collections",
        "collection_name,collection_description,collection_owner,collection_collaborators,collection_owner_name",
        placeholders.join(", "),
        "cast(collection_id as text) as collection_id,collection_name,cast(created_at as text) as created_at,cast(updated_at as text) as updated_at,collection_description,collection_owner,collection_collaborators,collection_owner_name");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.collection_name.as_ref().or(None)).bind(value.collection_description.as_ref().or(None)).bind(value.collection_owner.as_ref().or(None)).bind(value.collection_collaborators.as_ref().or(None)).bind(value.collection_owner_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/collections/{collection_id}")] async fn
get_collections_by_collection_id_handler(path : web :: Path < collections > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < collections > , sqlx :: Error > = collections ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/collections/{collection_id}")] async fn
patch_collections_by_collection_id_handler(path : web :: Path < collections >
, json : web :: Json < collections > , pool : web :: Data < PgPool >) -> impl
Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = collections :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/collections")] async fn
get_collections_handler(info : web :: Query < collections > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < collections > , sqlx
    :: Error > = collections :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/collections")] async fn get_collections_handler_schema()
-> impl Responder
{
    let b = schema_for! (collections); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/collections")] async fn
post_collections_by_collection_id_handler(json : web :: Json < collections > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    collections :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = collections :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/collections/{collection_id}")] async fn
delete_collections_by_collection_id_handler(path : web :: Path < collections >
, pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = collections
    :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct comments
{
    pub comment_id : Option < String > , pub comment_value : Option < String >
    , pub item_id : Option < String > , pub created_by : Option < String > ,
    pub created_at : Option < String > , pub creator_name : Option < String >
    , pub updated_at : Option < String > ,
} impl Schema for comments
{
    fn schema() -> String
    {
        let schema = schema_for! (comments); serde_json ::
        to_string(& schema).unwrap()
    }
} impl comments
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.comment_id.is_none()
        {
            fields.push(("comment_id", & self.comment_id as & dyn std :: fmt
            :: Debug));
        } if ! self.comment_value.is_none()
        {
            fields.push(("comment_value", & self.comment_value as & dyn std ::
            fmt :: Debug));
        } if ! self.item_id.is_none()
        {
            fields.push(("item_id", & self.item_id as & dyn std :: fmt ::
            Debug));
        } if ! self.created_by.is_none()
        {
            fields.push(("created_by", & self.created_by as & dyn std :: fmt
            :: Debug));
        } if ! self.created_at.is_none()
        {
            fields.push(("created_at", & self.created_at as & dyn std :: fmt
            :: Debug));
        } if ! self.creator_name.is_none()
        {
            fields.push(("creator_name", & self.creator_name as & dyn std ::
            fmt :: Debug));
        } if ! self.updated_at.is_none()
        {
            fields.push(("updated_at", & self.updated_at as & dyn std :: fmt
            :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.comment_id.is_none()
        { fields.push("comment_id"); } if ! self.comment_value.is_none()
        { fields.push("comment_value"); } if ! self.item_id.is_none()
        { fields.push("item_id"); } if ! self.created_by.is_none()
        { fields.push("created_by"); } if ! self.created_at.is_none()
        { fields.push("created_at"); } if ! self.creator_name.is_none()
        { fields.push("creator_name"); } if ! self.updated_at.is_none()
        { fields.push("updated_at"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.comment_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "comment_id", count));
        } if ! self.comment_value.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "comment_value", count));
        } if ! self.item_id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "item_id", count)); }
        if ! self.created_by.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "created_by", count));
        } if ! self.created_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "created_at", count));
        } if ! self.creator_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "creator_name", count));
        } if ! self.updated_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "updated_at", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for comments
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "comments", "comment_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.comment_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for comments
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["comment_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "comments")); if
        value.comment_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "comment_id")).push_bind(Uuid ::
            parse_str(value.comment_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.comment_value.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "comment_value")).push_bind(value.comment_value.as_ref().unwrap());
            first = false;
        }; if value.item_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "item_id")).push_bind(Uuid ::
            parse_str(value.item_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.created_by.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "created_by")).push_bind(value.created_by.as_ref().unwrap());
            first = false;
        }; if value.created_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "created_at")).push_bind(value.created_at.as_ref().unwrap());
            first = false;
        }; if value.creator_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "creator_name")).push_bind(value.creator_name.as_ref().unwrap());
            first = false;
        }; if value.updated_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "updated_at")).push_bind(value.updated_at.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "comment_id")).push_bind(Uuid ::
        parse_str(v.comment_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for comments
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < comments > , sqlx :: Error > = sqlx ::
        query_as :: < _, comments >
        ("SELECT cast(comment_id as varchar) as comment_id,comment_value,cast(item_id as varchar) as item_id,created_by,cast(created_at as varchar) as created_at,creator_name,cast(updated_at as varchar) as updated_at FROM comments WHERE comment_id = $1").bind(Uuid
        ::
        parse_str(v.comment_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for comments
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(comment_id as varchar) as comment_id,comment_value,cast(item_id as varchar) as item_id,created_by,cast(created_at as varchar) as created_at,creator_name,cast(updated_at as varchar) as updated_at FROM comments".to_string());
        println!
        ("{}",
        "SELECT cast(comment_id as varchar) as comment_id,comment_value,cast(item_id as varchar) as item_id,created_by,cast(created_at as varchar) as created_at,creator_name,cast(updated_at as varchar) as updated_at FROM comments");
        let final_query : sqlx :: query :: QueryAs < _, comments, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for comments
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(comment_id as varchar) as comment_id,comment_value,cast(item_id as varchar) as item_id,created_by,cast(created_at as varchar) as created_at,creator_name,cast(updated_at as varchar) as updated_at FROM comments".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.comment_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "comment_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.comment_id.as_ref().unwrap()).unwrap());
        }; if value.comment_value.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "comment_value")); count += 1;
            sqlx_query.push_bind(value.comment_value.as_ref().unwrap());
        }; if value.item_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "item_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.item_id.as_ref().unwrap()).unwrap());
        }; if value.created_by.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "created_by")); count += 1;
            sqlx_query.push_bind(value.created_by.as_ref().unwrap());
        }; if value.created_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "created_at")); count += 1;
            sqlx_query.push_bind(value.created_at.as_ref().unwrap());
        }; if value.creator_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "creator_name")); count += 1;
            sqlx_query.push_bind(value.creator_name.as_ref().unwrap());
        }; if value.updated_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "updated_at")); count += 1;
            sqlx_query.push_bind(value.updated_at.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, comments, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for comments
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 4usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "comments",
        "comment_value,item_id,created_by,creator_name",
        placeholders.join(", "),
        "cast(comment_id as text) as comment_id,comment_value,cast(item_id as text) as item_id,created_by,cast(created_at as text) as created_at,creator_name,cast(updated_at as text) as updated_at");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.comment_value.as_ref().or(None)).bind(Uuid ::
        parse_str(value.item_id.as_ref().unwrap()).unwrap()).bind(value.created_by.as_ref().or(None)).bind(value.creator_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/comments/{comment_id}")] async fn
get_comments_by_comment_id_handler(path : web :: Path < comments > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < comments > , sqlx :: Error > = comments :: select_where(p, v).await;
    match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/comments/{comment_id}")] async fn
patch_comments_by_comment_id_handler(path : web :: Path < comments > , json :
web :: Json < comments > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = comments :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/comments")] async fn
get_comments_handler(info : web :: Query < comments > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < comments > , sqlx ::
    Error > = comments :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/comments")] async fn get_comments_handler_schema() ->
impl Responder
{
    let b = schema_for! (comments); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/comments")] async fn
post_comments_by_comment_id_handler(json : web :: Json < comments > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = comments
    :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = comments :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/comments/{comment_id}")] async fn
delete_comments_by_comment_id_handler(path : web :: Path < comments > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = comments ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct entities
{ pub id : Option < i32 > , pub name : Option < String > , } impl Schema for
entities
{
    fn schema() -> String
    {
        let schema = schema_for! (entities); serde_json ::
        to_string(& schema).unwrap()
    }
} impl entities
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push(("id", & self.id as & dyn std :: fmt :: Debug)); } if !
        self.name.is_none()
        { fields.push(("name", & self.name as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push("id"); } if ! self.name.is_none()
        { fields.push("name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "id", count)); } if !
        self.name.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "name", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for entities
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "entities", "id = $1"); let sqlx_query :
        sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx :: query(& query).bind(v.id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for entities
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "entities")); if value.id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "id")).push_bind(value.id.as_ref().unwrap()); first =
            false;
        }; if value.name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "name")).push_bind(value.name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "id")).push_bind(v.id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for entities
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < entities > , sqlx :: Error > = sqlx ::
        query_as :: < _, entities >
        ("SELECT id,name FROM entities WHERE id = $1").bind(v.id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for entities
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,name FROM entities".to_string()); println!
        ("{}", "SELECT id,name FROM entities"); let final_query : sqlx ::
        query :: QueryAs < _, entities, _ > = sqlx_query.build_query_as();
        final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for entities
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,name FROM entities".to_string()); let mut count = 0; if
        value.where_clause().len() > 0 { sqlx_query.push(" where "); } if
        value.id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "id")); count += 1;
            sqlx_query.push_bind(value.id.as_ref().unwrap());
        }; if value.name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "name")); count += 1;
            sqlx_query.push_bind(value.name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, entities, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for entities
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 1usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "entities", "name",
        placeholders.join(", "), "id,name"); println! ("{}", query); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(value.name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/entities/{id}")] async fn
get_entities_by_id_handler(path : web :: Path < entities > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < entities > , sqlx :: Error > = entities :: select_where(p, v).await;
    match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/entities/{id}")] async fn
patch_entities_by_id_handler(path : web :: Path < entities > , json : web ::
Json < entities > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = entities :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/entities")] async fn
get_entities_handler(info : web :: Query < entities > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < entities > , sqlx ::
    Error > = entities :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/entities")] async fn get_entities_handler_schema() ->
impl Responder
{
    let b = schema_for! (entities); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/entities")] async fn
post_entities_by_id_handler(json : web :: Json < entities > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = entities
    :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = entities :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/entities/{id}")] async fn
delete_entities_by_id_handler(path : web :: Path < entities > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = entities ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct entityattributes
{
    pub eid : Option < String > , pub aid : Option < i32 > , pub vid : Option
    < i32 > ,
} impl Schema for entityattributes
{
    fn schema() -> String
    {
        let schema = schema_for! (entityattributes); serde_json ::
        to_string(& schema).unwrap()
    }
} impl entityattributes
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.eid.is_none()
        { fields.push(("eid", & self.eid as & dyn std :: fmt :: Debug)); } if
        ! self.aid.is_none()
        { fields.push(("aid", & self.aid as & dyn std :: fmt :: Debug)); } if
        ! self.vid.is_none()
        { fields.push(("vid", & self.vid as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.eid.is_none()
        { fields.push("eid"); } if ! self.aid.is_none()
        { fields.push("aid"); } if ! self.vid.is_none()
        { fields.push("vid"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.eid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "eid", count)); } if
        ! self.aid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "aid", count)); } if
        ! self.vid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "vid", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for entityattributes
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "entityattributes",
        "eid = $1 AND aid = $2 AND vid = $3"); let sqlx_query : sqlx :: query
        :: Query < sqlx :: Postgres, sqlx :: postgres :: PgArguments > = sqlx
        ::
        query(&
        query).bind(Uuid ::
        parse_str(v.eid.as_ref().unwrap()).unwrap()).bind(v.aid.as_ref().unwrap()).bind(v.vid.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for entityattributes
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["eid", "aid", "vid"]; println! ("Entering update");
        let active_fields : Vec < & str > = value.non_null_field_names(); let
        mut first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "entityattributes")); if
        value.eid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "eid")).push_bind(Uuid ::
            parse_str(value.eid.as_ref().unwrap()).unwrap()); first = false;
        }; if value.aid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "aid")).push_bind(value.aid.as_ref().unwrap()); first
            = false;
        }; if value.vid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "vid")).push_bind(value.vid.as_ref().unwrap()); first
            = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "eid")).push_bind(Uuid ::
        parse_str(v.eid.as_ref().unwrap()).unwrap());
        sqlx_query.push(format!
        (" and {} = ", "aid")).push_bind(v.aid.as_ref().unwrap());
        sqlx_query.push(format!
        (" and {} = ", "vid")).push_bind(v.vid.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for entityattributes
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < entityattributes > , sqlx :: Error > = sqlx
        :: query_as :: < _, entityattributes >
        ("SELECT cast(eid as varchar) as eid,aid,vid FROM entityattributes WHERE eid = $1 AND aid = $2 AND vid = $3").bind(Uuid
        ::
        parse_str(v.eid.as_ref().unwrap()).unwrap()).bind(v.aid.as_ref().unwrap()).bind(v.vid.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for entityattributes
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(eid as varchar) as eid,aid,vid FROM entityattributes".to_string());
        println!
        ("{}",
        "SELECT cast(eid as varchar) as eid,aid,vid FROM entityattributes");
        let final_query : sqlx :: query :: QueryAs < _, entityattributes, _ >
        = sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for entityattributes
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(eid as varchar) as eid,aid,vid FROM entityattributes".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.eid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "eid")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.eid.as_ref().unwrap()).unwrap());
        }; if value.aid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "aid")); count += 1;
            sqlx_query.push_bind(value.aid.as_ref().unwrap());
        }; if value.vid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "vid")); count += 1;
            sqlx_query.push_bind(value.vid.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, entityattributes, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for entityattributes
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 3usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "entityattributes",
        "eid,aid,vid", placeholders.join(", "),
        "cast(eid as text) as eid,aid,vid"); println! ("{}", query); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(value.eid.as_ref().unwrap()).unwrap()).bind(value.aid.as_ref().or(None)).bind(value.vid.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/entityattributes/{eid}/{aid}/{vid}")] async fn
get_entityattributes_by_eid_and_aid_and_vid_handler(path : web :: Path <
entityattributes > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < entityattributes > , sqlx :: Error > = entityattributes ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/entityattributes/{eid}/{aid}/{vid}")] async fn
patch_entityattributes_by_eid_and_aid_and_vid_handler(path : web :: Path <
entityattributes > , json : web :: Json < entityattributes > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = entityattributes :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/entityattributes")] async fn
get_entityattributes_handler(info : web :: Query < entityattributes > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < entityattributes > ,
    sqlx :: Error > = entityattributes :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/entityattributes")] async fn
get_entityattributes_handler_schema() -> impl Responder
{
    let b = schema_for! (entityattributes); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/entityattributes")] async fn
post_entityattributes_by_eid_and_aid_and_vid_handler(json : web :: Json <
entityattributes > , pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    entityattributes :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = entityattributes :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/entityattributes/{eid}/{aid}/{vid}")] async fn
delete_entityattributes_by_eid_and_aid_and_vid_handler(path : web :: Path <
entityattributes > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res =
    entityattributes :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct nominations
{
    pub nomination_id : Option < String > , pub asset_id : Option < String > ,
    pub nominator : Option < String > , pub features : Option < String > , pub
    impact : Option < String > , pub evidence : Option < String > , pub
    conclusion : Option < String > , pub created_at : Option < String > , pub
    status : Option < String > , pub nominator_name : Option < String > ,
} impl Schema for nominations
{
    fn schema() -> String
    {
        let schema = schema_for! (nominations); serde_json ::
        to_string(& schema).unwrap()
    }
} impl nominations
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.nomination_id.is_none()
        {
            fields.push(("nomination_id", & self.nomination_id as & dyn std ::
            fmt :: Debug));
        } if ! self.asset_id.is_none()
        {
            fields.push(("asset_id", & self.asset_id as & dyn std :: fmt ::
            Debug));
        } if ! self.nominator.is_none()
        {
            fields.push(("nominator", & self.nominator as & dyn std :: fmt ::
            Debug));
        } if ! self.features.is_none()
        {
            fields.push(("features", & self.features as & dyn std :: fmt ::
            Debug));
        } if ! self.impact.is_none()
        {
            fields.push(("impact", & self.impact as & dyn std :: fmt ::
            Debug));
        } if ! self.evidence.is_none()
        {
            fields.push(("evidence", & self.evidence as & dyn std :: fmt ::
            Debug));
        } if ! self.conclusion.is_none()
        {
            fields.push(("conclusion", & self.conclusion as & dyn std :: fmt
            :: Debug));
        } if ! self.created_at.is_none()
        {
            fields.push(("created_at", & self.created_at as & dyn std :: fmt
            :: Debug));
        } if ! self.status.is_none()
        {
            fields.push(("status", & self.status as & dyn std :: fmt ::
            Debug));
        } if ! self.nominator_name.is_none()
        {
            fields.push(("nominator_name", & self.nominator_name as & dyn std
            :: fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.nomination_id.is_none()
        { fields.push("nomination_id"); } if ! self.asset_id.is_none()
        { fields.push("asset_id"); } if ! self.nominator.is_none()
        { fields.push("nominator"); } if ! self.features.is_none()
        { fields.push("features"); } if ! self.impact.is_none()
        { fields.push("impact"); } if ! self.evidence.is_none()
        { fields.push("evidence"); } if ! self.conclusion.is_none()
        { fields.push("conclusion"); } if ! self.created_at.is_none()
        { fields.push("created_at"); } if ! self.status.is_none()
        { fields.push("status"); } if ! self.nominator_name.is_none()
        { fields.push("nominator_name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.nomination_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "nomination_id", count));
        } if ! self.asset_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "asset_id", count));
        } if ! self.nominator.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "nominator", count));
        } if ! self.features.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "features", count));
        } if ! self.impact.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "impact", count)); }
        if ! self.evidence.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "evidence", count));
        } if ! self.conclusion.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "conclusion", count));
        } if ! self.created_at.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "created_at", count));
        } if ! self.status.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "status", count)); }
        if ! self.nominator_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "nominator_name", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for nominations
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "nominations", "nomination_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(v.nomination_id.as_ref().unwrap()).unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for nominations
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["nomination_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "nominations")); if
        value.nomination_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "nomination_id")).push_bind(Uuid ::
            parse_str(value.nomination_id.as_ref().unwrap()).unwrap()); first
            = false;
        }; if value.asset_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} = ",
            "asset_id")).push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap()); first =
            false;
        }; if value.nominator.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "nominator")).push_bind(value.nominator.as_ref().unwrap()); first
            = false;
        }; if value.features.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "features")).push_bind(value.features.as_ref().unwrap()); first =
            false;
        }; if value.impact.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "impact")).push_bind(value.impact.as_ref().unwrap());
            first = false;
        }; if value.evidence.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "evidence")).push_bind(value.evidence.as_ref().unwrap()); first =
            false;
        }; if value.conclusion.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "conclusion")).push_bind(value.conclusion.as_ref().unwrap());
            first = false;
        }; if value.created_at.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "created_at")).push_bind(value.created_at.as_ref().unwrap());
            first = false;
        }; if value.status.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "status")).push_bind(value.status.as_ref().unwrap());
            first = false;
        }; if value.nominator_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "nominator_name")).push_bind(value.nominator_name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "nomination_id")).push_bind(Uuid ::
        parse_str(v.nomination_id.as_ref().unwrap()).unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for nominations
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < nominations > , sqlx :: Error > = sqlx ::
        query_as :: < _, nominations >
        ("SELECT cast(nomination_id as varchar) as nomination_id,cast(asset_id as varchar) as asset_id,nominator,features,impact,evidence,conclusion,cast(created_at as varchar) as created_at,status,nominator_name FROM nominations WHERE nomination_id = $1").bind(Uuid
        ::
        parse_str(v.nomination_id.as_ref().unwrap()).unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for nominations
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(nomination_id as varchar) as nomination_id,cast(asset_id as varchar) as asset_id,nominator,features,impact,evidence,conclusion,cast(created_at as varchar) as created_at,status,nominator_name FROM nominations".to_string());
        println!
        ("{}",
        "SELECT cast(nomination_id as varchar) as nomination_id,cast(asset_id as varchar) as asset_id,nominator,features,impact,evidence,conclusion,cast(created_at as varchar) as created_at,status,nominator_name FROM nominations");
        let final_query : sqlx :: query :: QueryAs < _, nominations, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for nominations
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT cast(nomination_id as varchar) as nomination_id,cast(asset_id as varchar) as asset_id,nominator,features,impact,evidence,conclusion,cast(created_at as varchar) as created_at,status,nominator_name FROM nominations".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.nomination_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "nomination_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.nomination_id.as_ref().unwrap()).unwrap());
        }; if value.asset_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "asset_id")); count += 1;
            sqlx_query.push_bind(Uuid ::
            parse_str(value.asset_id.as_ref().unwrap()).unwrap());
        }; if value.nominator.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "nominator")); count += 1;
            sqlx_query.push_bind(value.nominator.as_ref().unwrap());
        }; if value.features.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "features")); count += 1;
            sqlx_query.push_bind(value.features.as_ref().unwrap());
        }; if value.impact.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "impact")); count += 1;
            sqlx_query.push_bind(value.impact.as_ref().unwrap());
        }; if value.evidence.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "evidence")); count += 1;
            sqlx_query.push_bind(value.evidence.as_ref().unwrap());
        }; if value.conclusion.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "conclusion")); count += 1;
            sqlx_query.push_bind(value.conclusion.as_ref().unwrap());
        }; if value.created_at.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "created_at")); count += 1;
            sqlx_query.push_bind(value.created_at.as_ref().unwrap());
        }; if value.status.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "status")); count += 1;
            sqlx_query.push_bind(value.status.as_ref().unwrap());
        }; if value.nominator_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "nominator_name")); count += 1;
            sqlx_query.push_bind(value.nominator_name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, nominations, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for nominations
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 8usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "nominations",
        "asset_id,nominator,features,impact,evidence,conclusion,status,nominator_name",
        placeholders.join(", "),
        "cast(nomination_id as text) as nomination_id,cast(asset_id as text) as asset_id,nominator,features,impact,evidence,conclusion,cast(created_at as text) as created_at,status,nominator_name");
        println! ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx
        :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(Uuid ::
        parse_str(value.asset_id.as_ref().unwrap()).unwrap()).bind(value.nominator.as_ref().or(None)).bind(value.features.as_ref().or(None)).bind(value.impact.as_ref().or(None)).bind(value.evidence.as_ref().or(None)).bind(value.conclusion.as_ref().or(None)).bind(value.status.as_ref().or(None)).bind(value.nominator_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/nominations/{nomination_id}")] async fn
get_nominations_by_nomination_id_handler(path : web :: Path < nominations > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < nominations > , sqlx :: Error > = nominations ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/nominations/{nomination_id}")] async fn
patch_nominations_by_nomination_id_handler(path : web :: Path < nominations >
, json : web :: Json < nominations > , pool : web :: Data < PgPool >) -> impl
Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = nominations :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/nominations")] async fn
get_nominations_handler(info : web :: Query < nominations > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < nominations > , sqlx
    :: Error > = nominations :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/nominations")] async fn get_nominations_handler_schema()
-> impl Responder
{
    let b = schema_for! (nominations); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/nominations")] async fn
post_nominations_by_nomination_id_handler(json : web :: Json < nominations > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    nominations :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = nominations :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/nominations/{nomination_id}")] async fn
delete_nominations_by_nomination_id_handler(path : web :: Path < nominations >
, pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = nominations
    :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct offering_types
{
    pub offering_type_id : Option < String > , pub offering_type_name : Option
    < String > ,
} impl Schema for offering_types
{
    fn schema() -> String
    {
        let schema = schema_for! (offering_types); serde_json ::
        to_string(& schema).unwrap()
    }
} impl offering_types
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.offering_type_id.is_none()
        {
            fields.push(("offering_type_id", & self.offering_type_id as & dyn
            std :: fmt :: Debug));
        } if ! self.offering_type_name.is_none()
        {
            fields.push(("offering_type_name", & self.offering_type_name as &
            dyn std :: fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.offering_type_id.is_none()
        { fields.push("offering_type_id"); } if !
        self.offering_type_name.is_none()
        { fields.push("offering_type_name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.offering_type_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "offering_type_id", count));
        } if ! self.offering_type_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "offering_type_name", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for offering_types
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "offering_types",
        "offering_type_id = $1"); let sqlx_query : sqlx :: query :: Query <
        sqlx :: Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(& query).bind(v.offering_type_id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for offering_types
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["offering_type_id"]; println! ("Entering update");
        let active_fields : Vec < & str > = value.non_null_field_names(); let
        mut first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "offering_types")); if
        value.offering_type_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "offering_type_id")).push_bind(value.offering_type_id.as_ref().unwrap());
            first = false;
        }; if value.offering_type_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "offering_type_name")).push_bind(value.offering_type_name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "offering_type_id")).push_bind(v.offering_type_id.as_ref().unwrap());
        ; println! ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for offering_types
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < offering_types > , sqlx :: Error > = sqlx ::
        query_as :: < _, offering_types >
        ("SELECT offering_type_id,offering_type_name FROM offering_types WHERE offering_type_id = $1").bind(v.offering_type_id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for offering_types
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT offering_type_id,offering_type_name FROM offering_types".to_string());
        println!
        ("{}",
        "SELECT offering_type_id,offering_type_name FROM offering_types"); let
        final_query : sqlx :: query :: QueryAs < _, offering_types, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for offering_types
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT offering_type_id,offering_type_name FROM offering_types".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.offering_type_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "offering_type_id")); count +=
            1; sqlx_query.push_bind(value.offering_type_id.as_ref().unwrap());
        }; if value.offering_type_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "offering_type_name")); count
            += 1;
            sqlx_query.push_bind(value.offering_type_name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, offering_types, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for offering_types
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "offering_types",
        "offering_type_id,offering_type_name", placeholders.join(", "),
        "offering_type_id,offering_type_name"); println! ("{}", query); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.offering_type_id.as_ref().or(None)).bind(value.offering_type_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/offering_types/{offering_type_id}")] async fn
get_offering_types_by_offering_type_id_handler(path : web :: Path <
offering_types > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < offering_types > , sqlx :: Error > = offering_types ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/offering_types/{offering_type_id}")] async fn
patch_offering_types_by_offering_type_id_handler(path : web :: Path <
offering_types > , json : web :: Json < offering_types > , pool : web :: Data
< PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = offering_types :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/offering_types")] async fn
get_offering_types_handler(info : web :: Query < offering_types > , pool : web
:: Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < offering_types > ,
    sqlx :: Error > = offering_types :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/offering_types")] async fn
get_offering_types_handler_schema() -> impl Responder
{
    let b = schema_for! (offering_types); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/offering_types")] async fn
post_offering_types_by_offering_type_id_handler(json : web :: Json <
offering_types > , pool : web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    offering_types :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = offering_types :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/offering_types/{offering_type_id}")] async fn
delete_offering_types_by_offering_type_id_handler(path : web :: Path <
offering_types > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res =
    offering_types :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct parent
{
    pub id : Option < i32 > , pub paid : Option < i32 > , pub caid : Option <
    i32 > ,
} impl Schema for parent
{
    fn schema() -> String
    {
        let schema = schema_for! (parent); serde_json ::
        to_string(& schema).unwrap()
    }
} impl parent
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push(("id", & self.id as & dyn std :: fmt :: Debug)); } if !
        self.paid.is_none()
        { fields.push(("paid", & self.paid as & dyn std :: fmt :: Debug)); }
        if ! self.caid.is_none()
        { fields.push(("caid", & self.caid as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push("id"); } if ! self.paid.is_none()
        { fields.push("paid"); } if ! self.caid.is_none()
        { fields.push("caid"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "id", count)); } if !
        self.paid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "paid", count)); } if
        ! self.caid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "caid", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for parent
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format! ("DELETE FROM {} WHERE {}", "parent", "id = $1");
        let sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(v.id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for parent
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "parent")); if value.id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "id")).push_bind(value.id.as_ref().unwrap()); first =
            false;
        }; if value.paid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "paid")).push_bind(value.paid.as_ref().unwrap());
            first = false;
        }; if value.caid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "caid")).push_bind(value.caid.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "id")).push_bind(v.id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for parent
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < parent > , sqlx :: Error > = sqlx ::
        query_as :: < _, parent >
        ("SELECT id,paid,caid FROM parent WHERE id = $1").bind(v.id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for parent
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,paid,caid FROM parent".to_string()); println!
        ("{}", "SELECT id,paid,caid FROM parent"); let final_query : sqlx ::
        query :: QueryAs < _, parent, _ > = sqlx_query.build_query_as();
        final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for parent
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,paid,caid FROM parent".to_string()); let mut count = 0;
        if value.where_clause().len() > 0 { sqlx_query.push(" where "); } if
        value.id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "id")); count += 1;
            sqlx_query.push_bind(value.id.as_ref().unwrap());
        }; if value.paid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "paid")); count += 1;
            sqlx_query.push_bind(value.paid.as_ref().unwrap());
        }; if value.caid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "caid")); count += 1;
            sqlx_query.push_bind(value.caid.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, parent, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for parent
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "parent",
        "paid,caid", placeholders.join(", "), "id,paid,caid"); println!
        ("{}", query); let sqlx_query : sqlx :: query :: Query < sqlx ::
        Postgres, sqlx :: postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.paid.as_ref().or(None)).bind(value.caid.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/parent/{id}")] async fn
get_parent_by_id_handler(path : web :: Path < parent > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < parent > , sqlx :: Error > = parent :: select_where(p, v).await; match
    res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/parent/{id}")] async fn
patch_parent_by_id_handler(path : web :: Path < parent > , json : web :: Json
< parent > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = parent :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/parent")] async fn
get_parent_handler(info : web :: Query < parent > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < parent > , sqlx ::
    Error > = parent :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/parent")] async fn get_parent_handler_schema() -> impl
Responder
{
    let b = schema_for! (parent); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/parent")] async fn
post_parent_by_id_handler(json : web :: Json < parent > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = parent ::
    insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = parent :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/parent/{id}")] async fn
delete_parent_by_id_handler(path : web :: Path < parent > , pool : web :: Data
< PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = parent ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct practices
{
    pub practice_id : Option < String > , pub practice_name : Option < String
    > , pub owning_brand : Option < String > ,
} impl Schema for practices
{
    fn schema() -> String
    {
        let schema = schema_for! (practices); serde_json ::
        to_string(& schema).unwrap()
    }
} impl practices
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.practice_id.is_none()
        {
            fields.push(("practice_id", & self.practice_id as & dyn std :: fmt
            :: Debug));
        } if ! self.practice_name.is_none()
        {
            fields.push(("practice_name", & self.practice_name as & dyn std ::
            fmt :: Debug));
        } if ! self.owning_brand.is_none()
        {
            fields.push(("owning_brand", & self.owning_brand as & dyn std ::
            fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.practice_id.is_none()
        { fields.push("practice_id"); } if ! self.practice_name.is_none()
        { fields.push("practice_name"); } if ! self.owning_brand.is_none()
        { fields.push("owning_brand"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.practice_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "practice_id", count));
        } if ! self.practice_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "practice_name", count));
        } if ! self.owning_brand.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "owning_brand", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for practices
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "practices", "practice_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(v.practice_id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for practices
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["practice_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "practices")); if
        value.practice_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "practice_id")).push_bind(value.practice_id.as_ref().unwrap());
            first = false;
        }; if value.practice_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "practice_name")).push_bind(value.practice_name.as_ref().unwrap());
            first = false;
        }; if value.owning_brand.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "owning_brand")).push_bind(value.owning_brand.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "practice_id")).push_bind(v.practice_id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for practices
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < practices > , sqlx :: Error > = sqlx ::
        query_as :: < _, practices >
        ("SELECT practice_id,practice_name,owning_brand FROM practices WHERE practice_id = $1").bind(v.practice_id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for practices
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT practice_id,practice_name,owning_brand FROM practices".to_string());
        println!
        ("{}",
        "SELECT practice_id,practice_name,owning_brand FROM practices"); let
        final_query : sqlx :: query :: QueryAs < _, practices, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for practices
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT practice_id,practice_name,owning_brand FROM practices".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.practice_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "practice_id")); count += 1;
            sqlx_query.push_bind(value.practice_id.as_ref().unwrap());
        }; if value.practice_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "practice_name")); count += 1;
            sqlx_query.push_bind(value.practice_name.as_ref().unwrap());
        }; if value.owning_brand.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "owning_brand")); count += 1;
            sqlx_query.push_bind(value.owning_brand.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, practices, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for practices
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 3usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "practices",
        "practice_id,practice_name,owning_brand", placeholders.join(", "),
        "practice_id,practice_name,owning_brand"); println! ("{}", query); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(&
        query).bind(value.practice_id.as_ref().or(None)).bind(value.practice_name.as_ref().or(None)).bind(value.owning_brand.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/practices/{practice_id}")] async fn
get_practices_by_practice_id_handler(path : web :: Path < practices > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < practices > , sqlx :: Error > = practices :: select_where(p, v).await;
    match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/practices/{practice_id}")] async fn
patch_practices_by_practice_id_handler(path : web :: Path < practices > , json
: web :: Json < practices > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = practices :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/practices")] async fn
get_practices_handler(info : web :: Query < practices > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < practices > , sqlx ::
    Error > = practices :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/practices")] async fn get_practices_handler_schema() ->
impl Responder
{
    let b = schema_for! (practices); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/practices")] async fn
post_practices_by_practice_id_handler(json : web :: Json < practices > , pool
: web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = practices
    :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = practices :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/practices/{practice_id}")] async fn
delete_practices_by_practice_id_handler(path : web :: Path < practices > ,
pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = practices ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct products
{
    pub product_id : Option < String > , pub product_name : Option < String >
    ,
} impl Schema for products
{
    fn schema() -> String
    {
        let schema = schema_for! (products); serde_json ::
        to_string(& schema).unwrap()
    }
} impl products
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.product_id.is_none()
        {
            fields.push(("product_id", & self.product_id as & dyn std :: fmt
            :: Debug));
        } if ! self.product_name.is_none()
        {
            fields.push(("product_name", & self.product_name as & dyn std ::
            fmt :: Debug));
        } fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.product_id.is_none()
        { fields.push("product_id"); } if ! self.product_name.is_none()
        { fields.push("product_name"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.product_id.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "product_id", count));
        } if ! self.product_name.is_none()
        {
            count += 1;
            fields.push(format! (" {} = ${} ", "product_name", count));
        } fields.join(" AND ")
    }
} #[async_trait] impl Delete for products
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "products", "product_id = $1"); let
        sqlx_query : sqlx :: query :: Query < sqlx :: Postgres, sqlx ::
        postgres :: PgArguments > = sqlx ::
        query(& query).bind(v.product_id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for products
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["product_id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "products")); if
        value.product_id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "product_id")).push_bind(value.product_id.as_ref().unwrap());
            first = false;
        }; if value.product_name.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ",
            "product_name")).push_bind(value.product_name.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ",
        "product_id")).push_bind(v.product_id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for products
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < products > , sqlx :: Error > = sqlx ::
        query_as :: < _, products >
        ("SELECT product_id,product_name FROM products WHERE product_id = $1").bind(v.product_id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for products
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT product_id,product_name FROM products".to_string());
        println! ("{}", "SELECT product_id,product_name FROM products"); let
        final_query : sqlx :: query :: QueryAs < _, products, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for products
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT product_id,product_name FROM products".to_string()); let
        mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.product_id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "product_id")); count += 1;
            sqlx_query.push_bind(value.product_id.as_ref().unwrap());
        }; if value.product_name.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "product_name")); count += 1;
            sqlx_query.push_bind(value.product_name.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, products, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for products
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 2usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "products",
        "product_id,product_name", placeholders.join(", "),
        "product_id,product_name"); println! ("{}", query); let sqlx_query :
        sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx ::
        query(&
        query).bind(value.product_id.as_ref().or(None)).bind(value.product_name.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/products/{product_id}")] async fn
get_products_by_product_id_handler(path : web :: Path < products > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < products > , sqlx :: Error > = products :: select_where(p, v).await;
    match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/products/{product_id}")] async fn
patch_products_by_product_id_handler(path : web :: Path < products > , json :
web :: Json < products > , pool : web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = products :: update(p, j, v).await; match
    result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/products")] async fn
get_products_handler(info : web :: Query < products > , pool : web :: Data <
PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < products > , sqlx ::
    Error > = products :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/products")] async fn get_products_handler_schema() ->
impl Responder
{
    let b = schema_for! (products); let mut response = HttpResponse :: Ok();
    response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/products")] async fn
post_products_by_product_id_handler(json : web :: Json < products > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result = products
    :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = products :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/products/{product_id}")] async fn
delete_products_by_product_id_handler(path : web :: Path < products > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = products ::
    delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[derive(Deserialize, Serialize, Debug, sqlx :: FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")] pub struct relationship
{
    pub id : Option < i32 > , pub paid : Option < i32 > , pub caid : Option <
    i32 > , pub pavid : Option < i32 > , pub cavid : Option < i32 > ,
} impl Schema for relationship
{
    fn schema() -> String
    {
        let schema = schema_for! (relationship); serde_json ::
        to_string(& schema).unwrap()
    }
} impl relationship
{
    pub fn non_null_fields(& self) -> Vec < (& str, & dyn std :: fmt :: Debug)
    >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push(("id", & self.id as & dyn std :: fmt :: Debug)); } if !
        self.paid.is_none()
        { fields.push(("paid", & self.paid as & dyn std :: fmt :: Debug)); }
        if ! self.caid.is_none()
        { fields.push(("caid", & self.caid as & dyn std :: fmt :: Debug)); }
        if ! self.pavid.is_none()
        { fields.push(("pavid", & self.pavid as & dyn std :: fmt :: Debug)); }
        if ! self.cavid.is_none()
        { fields.push(("cavid", & self.cavid as & dyn std :: fmt :: Debug)); }
        fields
    } pub fn non_null_field_names(& self) -> Vec < & str >
    {
        let mut fields = Vec :: new(); if ! self.id.is_none()
        { fields.push("id"); } if ! self.paid.is_none()
        { fields.push("paid"); } if ! self.caid.is_none()
        { fields.push("caid"); } if ! self.pavid.is_none()
        { fields.push("pavid"); } if ! self.cavid.is_none()
        { fields.push("cavid"); } fields
    } pub fn where_clause(& self) -> String
    {
        let mut fields = Vec :: new(); let mut count = 0; if !
        self.id.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "id", count)); } if !
        self.paid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "paid", count)); } if
        ! self.caid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "caid", count)); } if
        ! self.pavid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "pavid", count)); }
        if ! self.cavid.is_none()
        { count += 1; fields.push(format! (" {} = ${} ", "cavid", count)); }
        fields.join(" AND ")
    }
} #[async_trait] impl Delete for relationship
{
    async fn delete(pool : & PgPool, v : Self) -> Result < sqlx :: postgres ::
    PgQueryResult, sqlx :: Error >
    {
        let query = format!
        ("DELETE FROM {} WHERE {}", "relationship", "id = $1"); let sqlx_query
        : sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx :: query(& query).bind(v.id.as_ref().unwrap());
        sqlx_query.execute(pool).await
    }
} #[async_trait] impl Update for relationship
{
    async fn update(pool : & PgPool, value : Self, v : Self) -> Result < sqlx
    :: postgres :: PgQueryResult, sqlx :: Error >
    {
        let pkeys = vec! ["id"]; println! ("Entering update"); let
        active_fields : Vec < & str > = value.non_null_field_names(); let mut
        first = true; let mut sqlx_query = sqlx :: QueryBuilder ::
        new(format! ("UPDATE {} set ", "relationship")); if value.id.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "id")).push_bind(value.id.as_ref().unwrap()); first =
            false;
        }; if value.paid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "paid")).push_bind(value.paid.as_ref().unwrap());
            first = false;
        }; if value.caid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "caid")).push_bind(value.caid.as_ref().unwrap());
            first = false;
        }; if value.pavid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "pavid")).push_bind(value.pavid.as_ref().unwrap());
            first = false;
        }; if value.cavid.is_some()
        {
            if ! first { sqlx_query.push(","); }
            sqlx_query.push(format!
            (" {} =  ", "cavid")).push_bind(value.cavid.as_ref().unwrap());
            first = false;
        };
        sqlx_query.push(format!
        (" where {} = ", "id")).push_bind(v.id.as_ref().unwrap()); ; println!
        ("Update: {:?}", sqlx_query.sql()); let final_query =
        sqlx_query.build(); let result = final_query.execute(pool).await;
        result
    }
} #[async_trait] impl SelectWhere for relationship
{
    async fn select_where(pool : & PgPool, v : Self) -> Result < Vec < Self >
    , sqlx :: Error >
    {
        let rows : Result < Vec < relationship > , sqlx :: Error > = sqlx ::
        query_as :: < _, relationship >
        ("SELECT id,paid,caid,pavid,cavid FROM relationship WHERE id = $1").bind(v.id.as_ref().unwrap()).fetch_all(pool).await;
        rows
    }
} #[async_trait] impl SelectAll for relationship
{
    async fn select_all(pool : & PgPool) -> Result < Vec < Self > , sqlx ::
    Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,paid,caid,pavid,cavid FROM relationship".to_string());
        println! ("{}", "SELECT id,paid,caid,pavid,cavid FROM relationship");
        let final_query : sqlx :: query :: QueryAs < _, relationship, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Select for relationship
{
    async fn select(pool : & PgPool, value : Self) -> Result < Vec < Self > ,
    sqlx :: Error >
    {
        let mut sqlx_query = sqlx :: QueryBuilder ::
        new("SELECT id,paid,caid,pavid,cavid FROM relationship".to_string());
        let mut count = 0; if value.where_clause().len() > 0
        { sqlx_query.push(" where "); } if value.id.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "id")); count += 1;
            sqlx_query.push_bind(value.id.as_ref().unwrap());
        }; if value.paid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "paid")); count += 1;
            sqlx_query.push_bind(value.paid.as_ref().unwrap());
        }; if value.caid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "caid")); count += 1;
            sqlx_query.push_bind(value.caid.as_ref().unwrap());
        }; if value.pavid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "pavid")); count += 1;
            sqlx_query.push_bind(value.pavid.as_ref().unwrap());
        }; if value.cavid.is_some()
        {
            if count > 0 { sqlx_query.push(" AND "); }
            sqlx_query.push(format! (" {} = ", "cavid")); count += 1;
            sqlx_query.push_bind(value.cavid.as_ref().unwrap());
        }; println! ("SQL {}:{:?}", count, value.where_clause()); let
        final_query : sqlx :: query :: QueryAs < _, relationship, _ > =
        sqlx_query.build_query_as(); final_query.fetch_all(pool).await
    }
} #[async_trait] impl Insert for relationship
{
    async fn insert(pool : & PgPool, value : Self) -> Result < Vec < sqlx ::
    postgres :: PgRow > , sqlx :: Error >
    {
        let placeholders : Vec < String > =
        (1 ..= 4usize).map(| i | format! ("${}", i)).collect(); let query =
        format!
        ("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", "relationship",
        "paid,caid,pavid,cavid", placeholders.join(", "),
        "id,paid,caid,pavid,cavid"); println! ("{}", query); let sqlx_query :
        sqlx :: query :: Query < sqlx :: Postgres, sqlx :: postgres ::
        PgArguments > = sqlx ::
        query(&
        query).bind(value.paid.as_ref().or(None)).bind(value.caid.as_ref().or(None)).bind(value.pavid.as_ref().or(None)).bind(value.cavid.as_ref().or(None));
        sqlx_query.fetch_all(pool).await
    }
} #[get("/api/relationship/{id}")] async fn
get_relationship_by_id_handler(path : web :: Path < relationship > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res : Result < Vec
    < relationship > , sqlx :: Error > = relationship ::
    select_where(p, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[patch("/api/relationship/{id}")] async fn
patch_relationship_by_id_handler(path : web :: Path < relationship > , json :
web :: Json < relationship > , pool : web :: Data < PgPool >) -> impl
Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let j =
    json.into_inner(); let result = relationship :: update(p, j, v).await;
    match result
    {
        Ok(res) => { println! ("Query executed successfully: {:?}", res); }
        Err(e) => { println! ("Error executing query: {:?}", e); }
    } let response = HttpResponse :: Ok(); response
} #[get("/api/relationship")] async fn
get_relationship_handler(info : web :: Query < relationship > , pool : web ::
Data < PgPool >) -> impl Responder
{
    let v = info.into_inner(); let res : Result < Vec < relationship > , sqlx
    :: Error > = relationship :: select(& pool, v).await; match res
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("content-type", "application/json"));
            response.json(a)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
} #[get("/api/schemas/relationship")] async fn
get_relationship_handler_schema() -> impl Responder
{
    let b = schema_for! (relationship); let mut response = HttpResponse ::
    Ok(); response.insert_header(("content-type", "application/json"));
    response.json(b)
} #[post("/api/relationship")] async fn
post_relationship_by_id_handler(json : web :: Json < relationship > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let p = pool.get_ref(); let v = json.into_inner(); let result =
    relationship :: insert(p, v).await; match result
    {
        Ok(a) =>
        {
            let mut response = HttpResponse :: Ok();
            response.insert_header(("Content-Type", "application/json")); let
            mut response_body = Vec :: new(); for row in a
            {
                let js = relationship :: from_row(& row).unwrap();
                response_body.push(js)
            } response.json(response_body)
        } Err(e) =>
        {
            eprint! ("Unexpected error: {} ", e); HttpResponse ::
            InternalServerError().body(format!
            ("Internal server error: {}", e))
        }
    }
} #[delete("/api/relationship/{id}")] async fn
delete_relationship_by_id_handler(path : web :: Path < relationship > , pool :
web :: Data < PgPool >) -> impl Responder
{
    let v = path.into_inner(); let p = pool.get_ref(); let res = relationship
    :: delete(p, v).await; match res
    {
        Ok(_a) =>
        {
            HttpResponse ::
            Ok().content_type("application/json").body(r#"{"message": "Succesfully deleted."}"#)
        }, Err(e) =>
        {
            eprint! ("Query failed: {}", e); HttpResponse ::
            InternalServerError().body("Internal server error")
        }
    }
}mod tests
{
    use super :: * ; use std :: env; use actix_web ::
    { App, test, web, http :: StatusCode }; #[actix_rt :: test] async fn
    test_get_actions_by_action_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_actions_by_action_id_handler))).await; let req =
        test :: TestRequest ::
        get().uri("/api/actions/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_actions_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_actions_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/actions").to_request(); let resp = test
        :: call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_actions_by_action_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_actions_by_action_id_handler))).await; let
        test_data = actions
        {
            action_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            action_user : Some(String :: from("test")), created_at :
            Some(String :: from("2024-10-16 14:30:00")), related_asset :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            action_type : Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/actions").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_actions_by_action_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_actions_by_action_id_handler))).await;
        println! ("{}", "/api/actions/acde070d-8c4c-4f0d-9d8a-162843c10333");
        let req = test :: TestRequest ::
        delete().uri("/api/actions/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_asset_bookmarks_by_asset_id_and_email_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_bookmarks_by_asset_id_and_email_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/asset_bookmarks/acde070d-8c4c-4f0d-9d8a-162843c10333/test").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_asset_bookmarks_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_bookmarks_handler))).await; let req = test
        :: TestRequest :: get().uri("/api/asset_bookmarks").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_asset_bookmarks_by_asset_id_and_email_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_asset_bookmarks_by_asset_id_and_email_handler))).await;
        let test_data = asset_bookmarks
        {
            asset_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            email : Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/asset_bookmarks").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_asset_bookmarks_by_asset_id_and_email_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_asset_bookmarks_by_asset_id_and_email_handler))).await;
        println!
        ("{}",
        "/api/asset_bookmarks/acde070d-8c4c-4f0d-9d8a-162843c10333/test"); let
        req = test :: TestRequest ::
        delete().uri("/api/asset_bookmarks/acde070d-8c4c-4f0d-9d8a-162843c10333/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_asset_collection_by_asset_id_and_collection_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_collection_by_asset_id_and_collection_id_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/asset_collection/acde070d-8c4c-4f0d-9d8a-162843c10333/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_asset_collection_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_collection_handler))).await; let req =
        test :: TestRequest ::
        get().uri("/api/asset_collection").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_asset_collection_by_asset_id_and_collection_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_asset_collection_by_asset_id_and_collection_id_handler))).await;
        let test_data = asset_collection
        {
            asset_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            collection_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
        }; let req = test :: TestRequest ::
        post().uri("/api/asset_collection").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_asset_collection_by_asset_id_and_collection_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_asset_collection_by_asset_id_and_collection_id_handler))).await;
        println!
        ("{}",
        "/api/asset_collection/acde070d-8c4c-4f0d-9d8a-162843c10333/acde070d-8c4c-4f0d-9d8a-162843c10333");
        let req = test :: TestRequest ::
        delete().uri("/api/asset_collection/acde070d-8c4c-4f0d-9d8a-162843c10333/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_asset_product_by_asset_id_and_product_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_product_by_asset_id_and_product_id_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/asset_product/acde070d-8c4c-4f0d-9d8a-162843c10333/test").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_asset_product_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_product_handler))).await; let req = test
        :: TestRequest :: get().uri("/api/asset_product").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_asset_product_by_asset_id_and_product_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_asset_product_by_asset_id_and_product_id_handler))).await;
        let test_data = asset_product
        {
            asset_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            product_id : Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/asset_product").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_asset_product_by_asset_id_and_product_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_asset_product_by_asset_id_and_product_id_handler))).await;
        println!
        ("{}",
        "/api/asset_product/acde070d-8c4c-4f0d-9d8a-162843c10333/test"); let
        req = test :: TestRequest ::
        delete().uri("/api/asset_product/acde070d-8c4c-4f0d-9d8a-162843c10333/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_asset_ratings_by_rating_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_ratings_by_rating_id_handler))).await; let
        req = test :: TestRequest ::
        get().uri("/api/asset_ratings/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_asset_ratings_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_ratings_handler))).await; let req = test
        :: TestRequest :: get().uri("/api/asset_ratings").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_asset_ratings_by_rating_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_asset_ratings_by_rating_id_handler))).await;
        let test_data = asset_ratings
        {
            rating_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            rating_value : Some(999f64), createdby :
            Some(String :: from("test")), related_asset :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
        }; let req = test :: TestRequest ::
        post().uri("/api/asset_ratings").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_asset_ratings_by_rating_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_asset_ratings_by_rating_id_handler))).await;
        println!
        ("{}", "/api/asset_ratings/acde070d-8c4c-4f0d-9d8a-162843c10333"); let
        req = test :: TestRequest ::
        delete().uri("/api/asset_ratings/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_asset_types_by_type_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_types_by_type_id_handler))).await; let req
        = test :: TestRequest ::
        get().uri("/api/asset_types/test").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_asset_types_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_asset_types_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/asset_types").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_asset_types_by_type_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_asset_types_by_type_id_handler))).await; let
        test_data = asset_types
        {
            type_id : Some(String :: from("test")), type_name :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/asset_types").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_asset_types_by_type_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_asset_types_by_type_id_handler))).await;
        println! ("{}", "/api/asset_types/test"); let req = test ::
        TestRequest ::
        delete().uri("/api/asset_types/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_assets_by_asset_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_assets_by_asset_id_handler))).await; let req =
        test :: TestRequest ::
        get().uri("/api/assets/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_assets_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_assets_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/assets").to_request(); let resp = test
        :: call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_assets_by_asset_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_assets_by_asset_id_handler))).await; let
        test_data = assets
        {
            asset_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            asset_name : Some(String :: from("test")), asset_owner :
            Some(String :: from("test")), asset_description :
            Some(String :: from("test")), asset_type :
            Some(String :: from("test")), asset_link :
            Some(String :: from("test")), created_at :
            Some(String :: from("2024-10-16 14:30:00")), updated_at :
            Some(String :: from("2024-10-16 14:30:00")), asset_offering_type :
            Some(String :: from("test")), asset_brand :
            Some(String :: from("test")), asset_practice :
            Some(String :: from("test")), is_ip_cleared : Some(true),
            is_sellable : Some(true), asset_rating_avg : Some(999f64),
            asset_collaborators :
            Some(vec! ["test".to_string(), "test".to_string(),]),
            asset_owner_name : Some(String :: from("test")), asset_geo :
            Some(String :: from("test")), asset_market :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/assets").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_assets_by_asset_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_assets_by_asset_id_handler))).await; println!
        ("{}", "/api/assets/acde070d-8c4c-4f0d-9d8a-162843c10333"); let req =
        test :: TestRequest ::
        delete().uri("/api/assets/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_attributes_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_attributes_by_id_handler))).await; let req =
        test :: TestRequest :: get().uri("/api/attributes/999").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_attributes_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_attributes_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/attributes").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_attributes_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_attributes_by_id_handler))).await; let
        test_data = attributes
        { id : Some(999i32), name : Some(String :: from("test")), }; let req =
        test :: TestRequest ::
        post().uri("/api/attributes").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_attributes_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_attributes_by_id_handler))).await; println!
        ("{}", "/api/attributes/999"); let req = test :: TestRequest ::
        delete().uri("/api/attributes/999").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_attributevalues_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_attributevalues_by_id_handler))).await; let req
        = test :: TestRequest ::
        get().uri("/api/attributevalues/999").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_attributevalues_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_attributevalues_handler))).await; let req = test
        :: TestRequest :: get().uri("/api/attributevalues").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_attributevalues_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_attributevalues_by_id_handler))).await; let
        test_data = attributevalues
        {
            id : Some(999i32), aid : Some(999i32), value :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/attributevalues").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_attributevalues_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_attributevalues_by_id_handler))).await;
        println! ("{}", "/api/attributevalues/999"); let req = test ::
        TestRequest ::
        delete().uri("/api/attributevalues/999").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_brands_by_brand_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_brands_by_brand_id_handler))).await; let req =
        test :: TestRequest :: get().uri("/api/brands/test").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_brands_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_brands_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/brands").to_request(); let resp = test
        :: call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_brands_by_brand_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_brands_by_brand_id_handler))).await; let
        test_data = brands
        {
            brand_id : Some(String :: from("test")), brand_name :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/brands").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_brands_by_brand_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_brands_by_brand_id_handler))).await; println!
        ("{}", "/api/brands/test"); let req = test :: TestRequest ::
        delete().uri("/api/brands/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_collections_by_collection_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_collections_by_collection_id_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/collections/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_collections_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_collections_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/collections").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_collections_by_collection_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_collections_by_collection_id_handler))).await;
        let test_data = collections
        {
            collection_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            collection_name : Some(String :: from("test")), created_at :
            Some(String :: from("2024-10-16 14:30:00")), updated_at :
            Some(String :: from("2024-10-16 14:30:00")),
            collection_description : Some(String :: from("test")),
            collection_owner : Some(String :: from("test")),
            collection_collaborators :
            Some(vec! ["test".to_string(), "test".to_string(),]),
            collection_owner_name : Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/collections").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_collections_by_collection_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_collections_by_collection_id_handler))).await;
        println!
        ("{}", "/api/collections/acde070d-8c4c-4f0d-9d8a-162843c10333"); let
        req = test :: TestRequest ::
        delete().uri("/api/collections/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_comments_by_comment_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_comments_by_comment_id_handler))).await; let req
        = test :: TestRequest ::
        get().uri("/api/comments/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_comments_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_comments_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/comments").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_comments_by_comment_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_comments_by_comment_id_handler))).await; let
        test_data = comments
        {
            comment_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            comment_value : Some(String :: from("test")), item_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            created_by : Some(String :: from("test")), created_at :
            Some(String :: from("2024-10-16 14:30:00")), creator_name :
            Some(String :: from("test")), updated_at :
            Some(String :: from("2024-10-16 14:30:00")),
        }; let req = test :: TestRequest ::
        post().uri("/api/comments").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_comments_by_comment_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_comments_by_comment_id_handler))).await;
        println! ("{}", "/api/comments/acde070d-8c4c-4f0d-9d8a-162843c10333");
        let req = test :: TestRequest ::
        delete().uri("/api/comments/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_entities_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_entities_by_id_handler))).await; let req = test
        :: TestRequest :: get().uri("/api/entities/999").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_entities_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_entities_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/entities").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_entities_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_entities_by_id_handler))).await; let test_data
        = entities
        { id : Some(999i32), name : Some(String :: from("test")), }; let req =
        test :: TestRequest ::
        post().uri("/api/entities").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_entities_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_entities_by_id_handler))).await; println!
        ("{}", "/api/entities/999"); let req = test :: TestRequest ::
        delete().uri("/api/entities/999").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_entityattributes_by_eid_and_aid_and_vid_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_entityattributes_by_eid_and_aid_and_vid_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/entityattributes/acde070d-8c4c-4f0d-9d8a-162843c10333/999/999").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_entityattributes_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_entityattributes_handler))).await; let req =
        test :: TestRequest ::
        get().uri("/api/entityattributes").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_entityattributes_by_eid_and_aid_and_vid_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_entityattributes_by_eid_and_aid_and_vid_handler))).await;
        let test_data = entityattributes
        {
            eid :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")), aid
            : Some(999i32), vid : Some(999i32),
        }; let req = test :: TestRequest ::
        post().uri("/api/entityattributes").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_entityattributes_by_eid_and_aid_and_vid_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_entityattributes_by_eid_and_aid_and_vid_handler))).await;
        println!
        ("{}",
        "/api/entityattributes/acde070d-8c4c-4f0d-9d8a-162843c10333/999/999");
        let req = test :: TestRequest ::
        delete().uri("/api/entityattributes/acde070d-8c4c-4f0d-9d8a-162843c10333/999/999").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_nominations_by_nomination_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_nominations_by_nomination_id_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/nominations/acde070d-8c4c-4f0d-9d8a-162843c10333").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_nominations_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_nominations_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/nominations").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_nominations_by_nomination_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_nominations_by_nomination_id_handler))).await;
        let test_data = nominations
        {
            nomination_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            asset_id :
            Some(String :: from("acde070d-8c4c-4f0d-9d8a-162843c10333")),
            nominator : Some(String :: from("test")), features :
            Some(String :: from("test")), impact :
            Some(String :: from("test")), evidence :
            Some(String :: from("test")), conclusion :
            Some(String :: from("test")), created_at :
            Some(String :: from("2024-10-16 14:30:00")), status :
            Some(String :: from("test")), nominator_name :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/nominations").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_nominations_by_nomination_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_nominations_by_nomination_id_handler))).await;
        println!
        ("{}", "/api/nominations/acde070d-8c4c-4f0d-9d8a-162843c10333"); let
        req = test :: TestRequest ::
        delete().uri("/api/nominations/acde070d-8c4c-4f0d-9d8a-162843c10333").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_get_offering_types_by_offering_type_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_offering_types_by_offering_type_id_handler))).await;
        let req = test :: TestRequest ::
        get().uri("/api/offering_types/test").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_offering_types_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_offering_types_handler))).await; let req = test
        :: TestRequest :: get().uri("/api/offering_types").to_request(); let
        resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_offering_types_by_offering_type_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_offering_types_by_offering_type_id_handler))).await;
        let test_data = offering_types
        {
            offering_type_id : Some(String :: from("test")),
            offering_type_name : Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/offering_types").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_offering_types_by_offering_type_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_offering_types_by_offering_type_id_handler))).await;
        println! ("{}", "/api/offering_types/test"); let req = test ::
        TestRequest ::
        delete().uri("/api/offering_types/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_parent_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_parent_by_id_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/parent/999").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_parent_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_parent_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/parent").to_request(); let resp = test
        :: call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_parent_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_parent_by_id_handler))).await; let test_data =
        parent
        { id : Some(999i32), paid : Some(999i32), caid : Some(999i32), }; let
        req = test :: TestRequest ::
        post().uri("/api/parent").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_parent_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_parent_by_id_handler))).await; println!
        ("{}", "/api/parent/999"); let req = test :: TestRequest ::
        delete().uri("/api/parent/999").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_practices_by_practice_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_practices_by_practice_id_handler))).await; let
        req = test :: TestRequest ::
        get().uri("/api/practices/test").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_practices_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_practices_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/practices").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_post_practices_by_practice_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_practices_by_practice_id_handler))).await; let
        test_data = practices
        {
            practice_id : Some(String :: from("test")), practice_name :
            Some(String :: from("test")), owning_brand :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/practices").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_practices_by_practice_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_practices_by_practice_id_handler))).await;
        println! ("{}", "/api/practices/test"); let req = test :: TestRequest
        ::
        delete().uri("/api/practices/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_products_by_product_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_products_by_product_id_handler))).await; let req
        = test :: TestRequest :: get().uri("/api/products/test").to_request();
        let resp = test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_products_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_products_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/products").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_products_by_product_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_products_by_product_id_handler))).await; let
        test_data = products
        {
            product_id : Some(String :: from("test")), product_name :
            Some(String :: from("test")),
        }; let req = test :: TestRequest ::
        post().uri("/api/products").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn
    test_delete_products_by_product_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_products_by_product_id_handler))).await;
        println! ("{}", "/api/products/test"); let req = test :: TestRequest
        ::
        delete().uri("/api/products/test").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_relationship_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_relationship_by_id_handler))).await; let req =
        test :: TestRequest ::
        get().uri("/api/relationship/999").to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_get_relationship_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(get_relationship_handler))).await; let req = test ::
        TestRequest :: get().uri("/api/relationship").to_request(); let resp =
        test :: call_service(& app, req).await; let status =
        resp.status().clone(); let body = test :: read_body(resp).await;
        println! ("{:?}", body.clone()); assert_eq!
        (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_post_relationship_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(post_relationship_by_id_handler))).await; let
        test_data = relationship
        {
            id : Some(999i32), paid : Some(999i32), caid : Some(999i32), pavid
            : Some(999i32), cavid : Some(999i32),
        }; let req = test :: TestRequest ::
        post().uri("/api/relationship").set_json(&
        test_data).append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    } #[actix_rt :: test] async fn test_delete_relationship_by_id_handler()
    {
        let database_url = env ::
        var("DATABASE_URL").expect("DATABASE_URL must be set"); let pool =
        PgPool ::
        connect(& database_url).await.expect("Failed to create pool."); let
        app = test ::
        init_service(App ::
        new().app_data(web :: Data ::
        new(pool.clone())).service(web ::
        scope("").service(delete_relationship_by_id_handler))).await; println!
        ("{}", "/api/relationship/999"); let req = test :: TestRequest ::
        delete().uri("/api/relationship/999").append_header(("Content-type",
        "application/json")).to_request(); let resp = test ::
        call_service(& app, req).await; let status = resp.status().clone();
        let body = test :: read_body(resp).await; println!
        ("{:?}", body.clone()); assert_eq! (status, StatusCode :: OK);
    }
}