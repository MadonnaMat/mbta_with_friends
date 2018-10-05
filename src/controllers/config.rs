use crate::models::user::*;
use rocket_contrib::Json;

#[derive(Serialize)]
pub struct Config {
    user: Option<JsonUser>
}

#[get("/config", rank=1)]
pub fn config_logged_in(current_user: User) -> Result<Json<Config>, ()>  {
    Ok(Json(Config{
        user: Some(current_user.to_json_user())
    }))
}


#[get("/config", rank=2)]
pub fn config() -> Result<Json<Config>, ()>  {
    Ok(Json(Config{
        user: None
    }))
}
