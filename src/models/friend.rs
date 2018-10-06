use crate::schema::friends;

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name="friends"]
pub struct Friend {
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
}

#[derive(Insertable)]
#[table_name="friends"]
pub struct NewFriend<'a> {
    pub user_id: &'a i32,
    pub friend_id: &'a i32,
}

#[derive(Deserialize)]
pub struct NFriend {
    pub username: String,
}
