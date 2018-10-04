use crate::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
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
