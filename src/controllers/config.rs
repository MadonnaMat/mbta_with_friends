use std::env;
use crate::models::user::*;
use rocket_contrib::Json;

#[derive(Serialize)]
pub struct Config {
    api_key: String,
    user: Option<JsonUser>
}

#[get("/config", rank=1)]
pub fn config_logged_in(current_user: User) -> Result<Json<Config>, ()>  {
    let api_key = env::var("MBTA_API_KEY").expect("MBTA_API_KEY must be set");

    Ok(Json(Config{
        api_key: api_key,
        user: Some(current_user.to_json_user())
    }))
}


#[get("/config", rank=2)]
pub fn config() -> Result<Json<Config>, ()>  {
    let api_key = env::var("MBTA_API_KEY").expect("MBTA_API_KEY must be set");

    Ok(Json(Config{
        api_key: api_key,
        user: None
    }))
}
