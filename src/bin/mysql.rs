#![feature(test)]
extern crate test;

use hello_actix::*;

use test::Bencher;

/// [dependencies]
//  actix-web = "3.3.2"
//  dotenv = "0.15.0"
//  diesel = { version = "1.4.7", features = ["mysql"] }
fn main() {}


#[test]
fn insert_user() {
    use std::env;
    use dotenv;
    use schema::users as userTable;
    use schema::users::dsl::users;

    use diesel::prelude::*;
    use diesel::Insertable;
    use diesel::result::Error;

    dotenv::dotenv().ok();
    let mysql_url = env::var("DATABASE_URL").unwrap();
    let conn = MysqlConnection::establish(&mysql_url).unwrap();

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


#[bench]
fn user_detail(b: &mut Bencher) {
    use std::env;
    use dotenv;
    use models::User;
    use schema::users::*;
    use schema::users::dsl::{id, users};
    use diesel::ExpressionMethods;
    use diesel::prelude::*;

    println!("{:?}", all_columns);
    println!("{:?}", SqlType::default());

    dotenv::dotenv().ok();
    let mysql_url = env::var("DATABASE_URL").unwrap();
    let conn = MysqlConnection::establish(&mysql_url).unwrap();

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
fn insert_get_id() {
    use dotenv;
    use std::env;

    use diesel::prelude::*;
    use diesel::Insertable;
    use diesel::result::Error;

    use diesel::ExpressionMethods;
    use schema::users as userTable;
    use schema::users::dsl::{id, users};

    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let conn = MysqlConnection::establish(&url).unwrap();

    let new_id = conn.transaction::<i32, Error, _>(|| {
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

        let new_id: i32 = users
            .select(id)
            .order(id.desc())
            .first(&conn)
            .unwrap();

        Ok(new_id)
    }).ok().unwrap();

    println!("new insert id is: {}", new_id);

    println!("delete new id...");
    let row = diesel::delete(users.filter(id.eq(new_id))).execute(&conn).unwrap();
    assert_eq!(row, 1);
}

#[test]
fn batch_insert_users() {
    use dotenv;
    use std::env;

    use diesel::prelude::*;
    use diesel::Insertable;
    use diesel::result::Error;

    use schema::users as userTable;
    use schema::users::dsl::{users, id, name};

    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let conn = MysqlConnection::establish(&url).unwrap();

    #[derive(Insertable)]
    #[table_name = "userTable"]
    struct NewUser {
        name: String,
    }
    let create_users = vec![
        NewUser { name: "aaa".to_string() },
        NewUser { name: "bbb".to_string() },
    ];

    conn.transaction::<(), Error, _>(|| {
        let row = diesel::insert_into(users)
            .values(&create_users)
            .execute(&conn)
            .unwrap();
        assert_eq!(row, 2);

        let new_users = users
            .select((id, name))
            .order(id.desc())
            .limit(2)
            .load::<(i32, String)>(&conn).unwrap();
        println!("{:?}", new_users);
        assert_eq!(new_users[0].1, "bbb");
        assert_eq!(new_users[1].1, "aaa");

        Ok(())
    }).ok();
}