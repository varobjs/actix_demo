#![feature(test)]
extern crate test;

use hello_actix::*;
use test::Bencher;

/// [dependencies]
//  actix-web = "3.3.2"
//  dotenv = "0.15.0"
//  diesel = { version = "1.4.7", features = ["mysql", "r2d2"] }
fn main() {}

#[bench]
fn user_detail(b: &mut Bencher) {
    use std::env;
    use dotenv;

    use models::User;
    use schema::users::dsl::{id, users};

    use diesel::prelude::*;
    use diesel::ExpressionMethods;
    use diesel::r2d2::ConnectionManager;

    dotenv::dotenv().ok();
    let mysql_url = env::var("DATABASE_URL").unwrap();

    let manager: ConnectionManager<MysqlConnection> = diesel::r2d2::ConnectionManager::new(&mysql_url);
    let pool = diesel::r2d2::Pool::builder()
        .max_size(50)
        .build(manager)
        .unwrap();
    let conn = pool.get().unwrap();

    b.iter(|| {
        let res: User = users
            .filter(id.eq(1))
            .order(id.desc())
            .first(&conn)
            .unwrap();
        println!("{:?}", res);
        assert_eq!(User { id: 1, name: "deli".to_string(), is_deleted: 0 }, res);
    });
}

#[test]
fn insert_user() {
    use std::env;
    use dotenv;

    use schema::users as userTable;
    use schema::users::dsl::*;

    use diesel::prelude::*;
    use diesel::r2d2::ConnectionManager;
    use diesel::result::Error;
    use diesel::Insertable;

    dotenv::dotenv().ok();
    let mysql_url = env::var("DATABASE_URL").unwrap();

    let manager: ConnectionManager<MysqlConnection> = diesel::r2d2::ConnectionManager::new(&mysql_url);
    let pool = diesel::r2d2::Pool::builder()
        .max_size(50)
        .build(manager)
        .unwrap();
    let conn = pool.get().unwrap();

    conn.transaction::<(), _, _>(|| {
        #[derive(Insertable)]
        #[table_name = "userTable"]
        struct NewUser {
            name: String,
        }
        let create_user = NewUser { name: "deli".to_string() };
        let row = diesel::insert_into(users)
            .values(&create_user)
            .execute(&conn)
            .unwrap();
        assert_eq!(row, 1);

        Err(Error::RollbackTransaction)
    }).ok();
}
