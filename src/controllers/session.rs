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

use time::Duration;

const BAD_USER : &'static str = "Bad Username or Password";

#[delete("/session")]
pub fn delete_session(mut cookies: Cookies) -> Result<String, ()> {
    cookies.remove_private(Cookie::named("user_id"));
    Ok(String::from("Logged Out"))
}

#[post("/session", format="application/json", data="<form_user>")]
pub fn new_session(conn: DbConn, form_user: Json<FormUserJson>, mut cookies: Cookies) -> Result<Json<JsonUser>, NotFound<String>> {
    let Json(form_user) = form_user;

    let db_user = users
        .filter(username.eq(form_user.username))
        .first::<User>(&*conn);

    match db_user {
        Ok(db_user) => {

            let valid = verify(&form_user.password, &db_user.password).unwrap_or(false);

            if valid {

                let mut c = Cookie::new("user_id", format!("{}", db_user.id));

                c.set_max_age(Duration::weeks(4));

                cookies.add_private(c);

                Ok(Json(db_user.toJsonUser()))
            } else {
             Err(NotFound(BAD_USER.to_string()))
            }
        },
        Err(e) => Err(NotFound(BAD_USER.to_string()))
    }
}
