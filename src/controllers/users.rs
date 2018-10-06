extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};

use crate::pg_pool::DbConn;
use crate::models::user::*;
use crate::models::friend::*;
use crate::schema::users::dsl::*;
use crate::schema::users;
use crate::schema::friends::dsl::*;

use rocket::response::status::{NotFound, Custom};
use rocket::http::{Cookies};
use rocket::http::Status;
use rocket_contrib::Json;

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;

#[get("/users", rank=1)]
pub fn all(conn: DbConn, current_user: User) -> Result<Json<Vec<JsonFriend>>, NotFound<String>> {
    get_friends(conn, current_user)
}

#[get("/users", rank=2)]
pub fn all_bad() -> Result<(), Custom<String>> {
    Err(Custom(Status::Forbidden, "User Must Be Logged In".to_string()))
}

#[post("/friends", format="application/json", data="<find_username>", rank=1)]
pub fn add_friend(conn: DbConn, current_user: User, find_username: Json<NFriend>) -> Result<Json<Vec<JsonFriend>>, NotFound<String>>  {
    let Json(find_username) = find_username;

    let find_friend = users
        .filter(username.eq(find_username.username))
        .first::<User>(&*conn);

    let find_friend = match find_friend {
        Ok(find_friend) => find_friend,
        Err(_) => return Err(NotFound("Username Not Found".to_string()))
    };

    if find_friend.id == current_user.id {
        return Err(NotFound("Cannot Friend Self".to_string()))
    };

    let new_friend = NewFriend {
        user_id: &current_user.id,
        friend_id: &find_friend.id,
    };

    let new_friend = diesel::insert_into(friends)
        .values(&new_friend)
        .get_result::<Friend>(&*conn);

    match new_friend {
        Ok(_) => get_friends(conn, current_user),
        Err(e) => Err(NotFound(format!("{}", e))),
    }
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

fn get_friends(conn: DbConn, current_user: User) -> Result<Json<Vec<JsonFriend>>, NotFound<String>> {
    let friend_ids = friends.select(friend_id)
        .filter(user_id.eq(current_user.id))
        .load::<i32>(&*conn);

    let friend_ids = match friend_ids {
        Ok(friend_ids) => friend_ids,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    let conn_friends = users.filter(users::dsl::id.eq_any(friend_ids))
        .load::<User>(&*conn);

    let conn_friends = match conn_friends {
        Ok(conn_friends) => conn_friends,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    let connected_friend_ids = friends
        .select(user_id)
        .filter(friend_id.eq(current_user.id))
        .load::<i32>(&*conn);

    let connected_friend_ids = match connected_friend_ids {
        Ok(connected_friend_ids) => connected_friend_ids,
        Err(e) => return Err(NotFound(format!("{}", e)))
    };

    Ok(Json(conn_friends.iter().map(|usr| usr.to_json_friend(&connected_friend_ids) ).collect()))

}
