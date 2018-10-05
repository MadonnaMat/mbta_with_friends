use crate::models::user::*;
use crate::pg_pool::DbConn;
use rocket_contrib::Json;

#[derive(Serialize)]
pub struct Config {
    user: Option<User>
}

#[get("/config", rank=1)]
pub fn config_logged_in(conn: DbConn, loggedIn: User) -> Result<Json<Config>, ()>  {
    Ok(Json(Config{
        user: Some(loggedIn)
    }))
}


#[get("/config", rank=2)]
pub fn config(conn: DbConn) -> Result<Json<Config>, ()>  {
    Ok(Json(Config{
        user: None
    }))
}
