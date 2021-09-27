#![feature(test)]
extern crate test;

use hello_actix::*;

use test::Bencher;
use diesel::{MysqlConnection, Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::result::Error;
use std::convert::TryInto;

use models::users::{NewUser, User};
use schema::users::dsl::{id, name, users};

/// [dependencies]
///
///  actix-web = "3.3.2"
///
///  dotenv = "0.15.0"
///
///  diesel = { version = "1.4.7", features = ["mysql"] }
fn main() {}

fn insert_user(conn: &MysqlConnection, user_name: String) -> usize {
    diesel::insert_into(users)
        .values(&NewUser { name: user_name })
        .execute(conn)
        .unwrap()
}

fn insert_user_get_id(conn: &MysqlConnection, user_name: String) -> i32 {
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

fn batch_insert_user(conn: &MysqlConnection, names: Vec<String>) -> usize {
    let create_users: Vec<NewUser> = names
        .iter()
        .map(|s| NewUser { name: s.to_string() })
        .collect();

    diesel::insert_into(users)
        .values(&create_users)
        .execute(conn)
        .unwrap()
}

fn user_detail(conn: &MysqlConnection, user_id: i32) -> User {
    users
        .filter(id.eq(user_id))
        .first(conn)
        .unwrap()
}

mod tests {
    use hello_actix::mysql_connection;
    use diesel::prelude::*;
    use diesel::result::Error;
    use super::*;

    #[test]
    fn test_insert_user() {
        let conn = mysql_connection();

        conn.transaction::<(), _, _>(|| {
            let row = insert_user(&conn, "deli".to_string());
            assert_eq!(row, 1);

            Err(Error::RollbackTransaction)
        }).ok();
    }

    #[test]
    fn test_insert_user_get_id() {
        let conn = mysql_connection();

        conn.transaction::<(), _, _>(|| {
            let new_id = insert_user_get_id(&conn, "deli".to_string());
            assert!(new_id > 0);

            Err(Error::RollbackTransaction)
        }).ok();
    }

    #[test]
    fn test_batch_insert_user() {
        let conn = mysql_connection();

        conn.transaction::<(), _, _>(|| {
            let rows = batch_insert_user(&conn, vec!["first".to_string(), "second".to_string()]);
            assert_eq!(rows, 2);

            let names = users
                .select((id, name))
                .order(id.desc())
                .limit(rows.try_into().unwrap())
                .load::<(i32, String)>(&conn).unwrap();
            println!("{:?}", names);
            assert_eq!(names[0].1, "second".to_string());
            assert_eq!(names[1].1, "first".to_string());

            Err(Error::RollbackTransaction)
        }).ok();
    }

    #[bench]
    fn test_user_detail(b: &mut Bencher) {
        let conn = mysql_connection();
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