#![feature(test)]
extern crate test;

use hello_actix::*;
use test::Bencher;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::{MysqlConnection, Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::result::Error;

use schema::users::dsl::{id, users};
use models::users::NewUser;
use hello_actix::models::users::User;

/// [dependencies]
///
/// actix-web = "3.3.2"
///
/// dotenv = "0.15.0"
///
/// diesel = { version = "1.4.7", features = ["mysql", "r2d2"] }
///
///

fn main() {}

fn insert_user(conn: &PooledConnection<ConnectionManager<MysqlConnection>>, user_name: String) -> usize {
    diesel::insert_into(users)
        .values(&NewUser { name: user_name })
        .execute(conn)
        .unwrap()
}

fn insert_user_get_id(conn: &PooledConnection<ConnectionManager<MysqlConnection>>, user_name: String) -> i32 {
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

fn user_detail(conn: &PooledConnection<ConnectionManager<MysqlConnection>>, user_id: i32) -> User {
    users
        .filter(id.eq(user_id))
        .first(conn)
        .unwrap()
}

mod tests {
    use crate::*;

    #[test]
    fn test_insert_user() {
        let pool = r2d2_mysql_connection_pool();
        let conn = pool.get().unwrap();

        let row = insert_user(&conn, "deli".to_string());
        assert_eq!(row, 1);
    }

    #[test]
    fn test_insert_user_get_id() {
        let pool = r2d2_mysql_connection_pool();
        let conn = pool.get().unwrap();

        let new_id = insert_user_get_id(&conn, "deli".to_string());
        assert!(new_id > 0);
    }

    #[bench]
    fn test_user_detail(b: &mut Bencher) {
        let pool = r2d2_mysql_connection_pool();
        let conn = pool.get().unwrap();

        conn.transaction::<(), _, _>(|| {
            let new_id = insert_user_get_id(&conn, "deli".to_string());

            b.iter(|| {
                let user = user_detail(&conn, new_id);
                println!("{:?}", user);
                assert_eq!(User { id: new_id, name: "deli".to_string(), is_deleted: 0 }, user);
            });

            Err(Error::RollbackTransaction)
        }).ok();
    }
}
