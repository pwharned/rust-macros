impl ApiClient { fn get_host(& self) -> & str { & self.host } } use serde_json
:: Value; #[derive(Debug)] enum ApiError
{ Reqwest(reqwest :: Error), SerdeJson(serde_json :: Error), } impl fmt ::
Display for ApiError
{
    fn fmt(& self, f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
    {
        match self
        {
            ApiError :: Reqwest(e) => write! (f, "Reqwest error: {}", e),
            ApiError :: SerdeJson(e) => write! (f, "Serde JSON error: {}", e),
        }
    }
} impl From < reqwest :: Error > for ApiError
{ fn from(error : reqwest :: Error) -> Self { ApiError :: Reqwest(error) } }
impl From < serde_json :: Error > for ApiError
{
    fn from(error : serde_json :: Error) -> Self
    { ApiError :: SerdeJson(error) }
} impl ApiClient
{
    async fn get_token(& self, apikey : String) -> Result < String, ApiError >
    {
        let client = Client :: new(); let body = format!
        ("grant_type={}&{}={}", "urn:ibm:params:oauth:grant-type:apikey",
        "apikey", apikey); let response =
        client.post("https://iam.cloud.ibm.com/identity/token").header("Content-Type",
        "application/x-www-form-urlencoded").body(body).send().await ? ; let
        json : Value = response.json().await ? ;
        Ok(json.get("access_token").and_then(Value ::
        as_str).map(String :: from).unwrap())
    }
} impl ApiClient
{
    async fn
    post_ml_v1_text_generation_stream(& self, token : String, version :
    String, textgenrequest : TextGenRequest,) -> Result < String, ApiError >
    {
        let url = format!
        ("{}/ml/v1/text/generation_stream", self.get_host()); let client =
        Client :: new(); let params = [("version", version)]; let response =
        client.post(url).header("Authorization", format!
        ("Bearer: {}",
        token)).json(& textgenrequest).query(& params).send().await ? ; let
        mut bytes = Vec :: new(); let mut stream = response.bytes_stream();
        while let Some(event) = stream.next().await
        {
            match event
            {
                Ok(event_bytes) => bytes.extend_from_slice(& event_bytes),
                Err(e) => return Err(ApiError :: Reqwest(e)),
            }
        } let strings : Vec < String > =
        bytes.split(| & byte | byte ==
        0).filter(| slice | !
        slice.is_empty()).map(| slice |
        {
            let vec_u8 = slice.to_vec(); String ::
            from_utf8_lossy(& vec_u8).to_string()
        }).collect(); let data = strings.join(""); Ok(data)
    }
} impl ApiClient
{
    async fn
    post_ml_v1_text_generation(& self, token : String, version : String,
    textgenrequest : TextGenRequest,) -> Result < Value, ApiError >
    {
        let url = format! ("{}/ml/v1/text/generation", self.get_host()); let
        client = Client :: new(); let params = [("version", version)]; let
        response =
        client.post(url).header("Authorization", format!
        ("Bearer: {}",
        token)).json(& textgenrequest).query(& params).send().await ? ; let
        mut bytes = response.bytes().await.unwrap(); match serde_json ::
        from_slice :: < Value > (& bytes)
        { Ok(data) => Ok(data), Err(e) => Err(ApiError :: from(e)) }
    }
}mod tests
{
    use super :: * ; use std :: env; use actix_web ::
    { App, test, web, http :: StatusCode };
}