use rocket::request;
use rocket::request::FromRequest;
use rocket::http::Status;
use rocket::{Outcome, Request, State};

use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::pg_pool::Pool;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct JsonUser {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a>{
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct FormUserJson {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn toJsonUser(&self) -> JsonUser {
        JsonUser {
            id: self.id,
            username: String::from(&self.username[..])
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let mut cookies = request.cookies();
        let user_id = cookies.get_private("user_id");
        match user_id {
            Some(user_id) => {
                let user_id = user_id.value().to_string();
                let pool = request.guard::<State<Pool>>()?;
                match pool.get() {
                    Ok(conn) => {
                        let user_id = user_id.parse::<i32>().unwrap_or(-1);

                        let db_user = users
                            .find(user_id)
                            .first::<User>(&conn);

                        match db_user {
                            Ok(db_user) => Outcome::Success(db_user),
                            Err(_) => Outcome::Forward(())
                        }
                    },
                    Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
                }
            },
            None => Outcome::Forward(())
        }

    }
}
