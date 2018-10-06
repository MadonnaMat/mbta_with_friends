use rocket::request;
use rocket::request::FromRequest;
use rocket::{Outcome, Request};

use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::pg_pool::DbConn;
use rocket::http::{Cookies, Cookie};

use time::Duration;

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name="users"]
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

#[derive(Serialize)]
pub struct JsonFriend {
    pub id: i32,
    pub username: String,
    pub is_friend: bool,
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
    pub fn to_json_user(&self) -> JsonUser {
        JsonUser {
            id: self.id,
            username: String::from(&self.username[..])
        }
    }

    pub fn to_json_friend(&self, connected_friend_ids: &Vec<i32>) -> JsonFriend {
        JsonFriend {
            id: self.id,
            username: String::from(&self.username[..]),
            is_friend: connected_friend_ids.contains(&self.id),
        }
    }

    pub fn set_user_cookie(&self, mut cookies: Cookies) {
        let mut c = Cookie::new("user_id", format!("{}", self.id));

        c.set_max_age(Duration::weeks(4));

        cookies.add_private(c);
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let mut cookies = request.cookies();
        let user_id_cookie = cookies.get_private("user_id");

        let user_id = match user_id_cookie {
            Some(user_id) => user_id,
            None => return Outcome::Forward(())
        };

        let user_id = user_id.value().to_string();
        let conn = request.guard::<DbConn>()?;
        let user_id = user_id.parse::<i32>().unwrap_or(-1);

        let db_user = users
            .find(user_id)
            .first::<User>(&*conn);

        match db_user {
            Ok(db_user) => Outcome::Success(db_user),
            Err(_) => Outcome::Forward(())
        }

    }
}
