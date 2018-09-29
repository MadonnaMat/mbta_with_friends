use crate::pg_pool::DbConn;
use crate::models::user::*;
use crate::schema::users::dsl::*;

use rocket::response::status::NotFound;
use rocket_contrib::Json;

use diesel::RunQueryDsl;

#[get("/users")]
pub fn all(conn: DbConn) -> Result<Json<Vec<User>>, NotFound<String>> {
    let usrs = users.load::<User>(&*conn);

    match usrs {
        Ok(usrs) => Ok(Json(usrs)),
        Err(e) => Err(NotFound(format!("{}", e)))
    }
}

#[post("/users", format="application/json", data="<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUserJson>) -> Result<Json<User>, NotFound<String>> {
    let Json(new_user) = new_user;

    let new_user = NewUser {
        username: &new_user.username[..],
        password: &new_user.password[..],
    };

    let usr = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&*conn);

    match usr {
        Ok(usr) => Ok(Json(usr)),
        Err(e) => Err(NotFound(format!("{}", e)))
    }
}
