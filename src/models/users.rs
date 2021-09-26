use crate::schema::users;

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_deleted: i8,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}