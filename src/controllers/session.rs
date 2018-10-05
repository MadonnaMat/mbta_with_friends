extern crate bcrypt;

use bcrypt::verify;

use crate::pg_pool::DbConn;
use crate::models::user::*;
use crate::schema::users::dsl::*;

use rocket::response::status::NotFound;
use rocket_contrib::Json;
use rocket::http::{Cookies, Cookie};

use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

const BAD_USER : &'static str = "Bad Username or Password";

#[delete("/session")]
pub fn delete_session(mut cookies: Cookies) -> Result<String, ()> {
    cookies.remove_private(Cookie::named("user_id"));
    Ok(String::from("Logged Out"))
}

#[post("/session", format="application/json", data="<form_user>")]
pub fn new_session(conn: DbConn, form_user: Json<FormUserJson>, cookies: Cookies) -> Result<Json<JsonUser>, NotFound<String>> {
    let Json(form_user) = form_user;

    let db_user = users
        .filter(username.eq(form_user.username))
        .first::<User>(&*conn);

    let db_user = match db_user {
        Ok(db_user) => db_user,
        Err(_) => return Err(NotFound(BAD_USER.to_string()))
    };

    let valid = verify(&form_user.password, &db_user.password).unwrap_or(false);

    if valid {
        db_user.set_user_cookie(cookies);

        Ok(Json(db_user.to_json_user()))
    } else {
        Err(NotFound(BAD_USER.to_string()))
    }
}
