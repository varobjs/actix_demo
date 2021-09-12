#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_deleted: i8,
}