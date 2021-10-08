/// Cargo.toml
/// [dependencies]
/// actix-web = "3.3.2"
/// dotenv = "0.15.0"
/// diesel = { version = "1.4.7", features = ["mysql", "r2d2"] }

// use test::Bencher;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::{MysqlConnection, Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::result::Error;

use crate::schema::users::dsl::{id, users};
use crate::models::users::NewUser;
use crate::models::users::User;

/// 添加用户
///
/// Examples:
///
/// ```
/// use diesel::prelude::*;
/// use diesel::Connection;
/// use diesel::result::Error;
///
/// use hello_actix::r2d2_mysql_connection_pool;
/// use hello_actix::services::users_r2d2;
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let pool = r2d2_mysql_connection_pool(url);
/// let conn = pool.get().unwrap();
///
/// conn.transaction::<(), _, _>(|| {
///     let row = users_r2d2::insert_user(&conn, "deli".to_string());
///     assert_eq!(row, 1);
///
///     Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn insert_user(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    user_name: String,
) -> usize {
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
/// use hello_actix::r2d2_mysql_connection_pool;
/// use hello_actix::services::users_r2d2;
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let pool = r2d2_mysql_connection_pool(url);
/// let conn = pool.get().unwrap();
///
/// conn.transaction::<(), _, _>(|| {
///     let new_id = users_r2d2::insert_user_get_id(&conn, "deli".to_string());
///     assert!(new_id > 0);
///
///     Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn insert_user_get_id(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    user_name: String,
) -> i32 {
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

/// 查询用户详情
///
/// Examples:
///
/// ```
/// use diesel::prelude::*;
/// use diesel::Connection;
/// use diesel::result::Error;
///
/// use hello_actix::r2d2_mysql_connection_pool;
/// use hello_actix::services::users_r2d2;
/// use hello_actix::models::users::User;
///
/// dotenv::dotenv().ok();
/// let url = std::env::var("DATABASE_URL").unwrap();
/// let pool = r2d2_mysql_connection_pool(url);
/// let conn = pool.get().unwrap();
///
/// conn.transaction::<(), _, _>(|| {
///     let new_id = users_r2d2::insert_user_get_id(&conn, "deli".to_string());
///     let user = users_r2d2::user_detail(&conn, new_id).unwrap();
///     println!("{:?}", user);
///     assert_eq!(User { id: new_id, name: "deli".to_string(), is_deleted: 0 }, user);
///
///     Err(Error::RollbackTransaction)
/// }).ok();
/// ```
pub fn user_detail(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    user_id: i32,
) -> Result<User, Error> {
    users
        .filter(id.eq(user_id))
        .first::<User>(conn)
}