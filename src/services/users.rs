/// Cargo.toml
/// [dependencies]
/// actix-web = "3.3.2"
/// dotenv = "0.15.0"
/// diesel = { version = "1.4.7", features = ["mysql"] }

use diesel::{MysqlConnection, Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::result::Error;

use crate::models::users::{NewUser, User};
use crate::schema::users::dsl::{id, users};


/// 添加用户
///
/// Examples:
///
/// ```
/// use diesel::prelude::*;
/// use diesel::Connection;
/// use diesel::result::Error;
///
/// use hello_actix::mysql_connection;
/// use hello_actix::services::users;
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let conn = mysql_connection(url);
///
/// conn.transaction::<(), _, _>(|| {
///     let row = users::insert_user(&conn, "deli".to_string());
///     assert_eq!(row, 1);
///
///    Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn insert_user(conn: &MysqlConnection, user_name: String) -> usize {
    diesel::insert_into(users)
        .values(&NewUser { name: user_name })
        .execute(conn)
        .unwrap()
}

/// 添加用户，返回user_id
///
/// Examples:
///
/// ```
/// use diesel::prelude::*;
/// use diesel::Connection;
/// use diesel::result::Error;
///
/// use hello_actix::mysql_connection;
/// use hello_actix::services::users;
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let conn = mysql_connection(url);
///
/// conn.transaction::<(), _, _>(|| {
///     let new_id = users::insert_user_get_id(&conn, "deli".to_string());
///     assert!(new_id > 0);
///
///    Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn insert_user_get_id(conn: &MysqlConnection, user_name: String) -> i32 {
    conn.transaction::<i32, Error, _>(|| {
        insert_user(conn, user_name);

        let new_id: i32 = users
            .select(id)
            .order(id.desc())
            .first(conn)
            .unwrap();

        Ok(new_id)
    }).ok().unwrap()
}

/// 批量添加用户
///
/// Examples:
///
/// ```
/// use std::convert::TryInto;
/// use diesel::prelude::*;
/// use diesel::Connection;
/// use diesel::result::Error;
/// use diesel::ExpressionMethods;
///
/// use hello_actix::mysql_connection;
/// use hello_actix::services::users;
/// use hello_actix::models::users::User;
/// use hello_actix::schema::users::dsl::{id, name, users as SchemaUsers};
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let conn = mysql_connection(url);
///
/// conn.transaction::<(), _, _>(|| {
///     let rows = users::batch_insert_user(&conn, vec!["first".to_string(), "second".to_string()]);
///     assert_eq!(rows, 2);
///
///     let names = SchemaUsers
///                 .select((id, name))
///                 .order(id.desc())
///                 .limit(rows.try_into().unwrap())
///                 .load::<(i32, String)>(&conn).unwrap();
///     println!("{:?}", names);
///     assert_eq!(names[0].1, "second".to_string());
///     assert_eq!(names[1].1, "first".to_string());
///     Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn batch_insert_user(conn: &MysqlConnection, names: Vec<String>) -> usize {
    let create_users: Vec<NewUser> = names
        .iter()
        .map(|s| NewUser { name: s.to_string() })
        .collect();

    diesel::insert_into(users)
        .values(&create_users)
        .execute(conn)
        .unwrap()
}

/// 查询用户详情
///
/// Examples:
///
/// ```
/// use diesel::prelude::*;
/// use diesel::Connection;
/// use diesel::result::Error;
///
/// use hello_actix::mysql_connection;
/// use hello_actix::services::users;
/// use hello_actix::models::users::User;
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let conn = mysql_connection(url);
///
/// conn.transaction::<(), _, _>(|| {
///     let new_id = users::insert_user_get_id(&conn, "deli".to_string());
///     let user = users::user_detail(&conn, new_id);
///     println!("{:?}", user);
///     assert_eq!(User { id: new_id, name: "deli".to_string(), is_deleted: 0 }, user);
///
///    Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn user_detail(conn: &MysqlConnection, user_id: i32) -> User {
    users
        .filter(id.eq(user_id))
        .first(conn)
        .unwrap()
}
