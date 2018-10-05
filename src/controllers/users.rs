extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};

use crate::pg_pool::DbConn;
use crate::models::user::*;
use crate::schema::users::dsl::*;

use rocket::response::status::{NotFound, Custom};
use rocket::http::{Cookies};
use rocket::http::Status;
use rocket_contrib::Json;

use diesel::RunQueryDsl;

#[get("/users", rank=1)]
pub fn all(conn: DbConn, _current_user: User) -> Result<Json<Vec<JsonUser>>, NotFound<String>> {
    let usrs = users.load::<User>(&*conn);

    match usrs {
        Ok(usrs) => Ok(Json(usrs.iter().map(|usr| usr.to_json_user() ).collect())),
        Err(e) => Err(NotFound(format!("{}", e)))
    }
}

#[get("/users", rank=2)]
pub fn all_bad() -> Result<(), Custom<String>> {
    Err(Custom(Status::Forbidden, "User Must Be Logged In".to_string()))
}

#[post("/users", format="application/json", data="<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<FormUserJson>, cookies: Cookies) -> Result<Json<JsonUser>, NotFound<String>> {
    let Json(new_user) = new_user;

    let encrypted = hash(&new_user.password[..], DEFAULT_COST);

    let encrypted = match encrypted {
        Ok(encrypted) => encrypted,
        Err(_) => return Err(NotFound("Unknown Error!".to_string()))
    };

    let new_user = NewUser {
        username: &new_user.username[..],
        password: &encrypted[..],
    };

    let usr = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&*conn);

    match usr {
        Ok(usr) => {
            usr.set_user_cookie(cookies);

            Ok(Json(usr.to_json_user()))
        },
        Err(e) => Err(NotFound(format!("{}", e)))
    }
}   
