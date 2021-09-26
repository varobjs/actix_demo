#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn mysql_connection() -> MysqlConnection {
    dotenv::dotenv().ok();
    let mysql_url = std::env::var("DATABASE_URL").unwrap();
    MysqlConnection::establish(&mysql_url).unwrap()
}

pub fn r2d2_mysql_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv::dotenv().ok();
    let mysql_url = std::env::var("DATABASE_URL").unwrap();

    let manager: ConnectionManager<MysqlConnection> = diesel::r2d2::ConnectionManager::new(&mysql_url);
    diesel::r2d2::Pool::builder()
        .max_size(50)
        .build(manager)
        .unwrap()
}

mod tests {
    #[test]
    fn test_mysql_connection() {
        use crate::*;

        let conn = mysql_connection();
        let row = conn.execute("select 1").unwrap();
        assert_eq!(row, 1);
    }

    #[test]
    fn test_r2d2_mysql_connection_pool() {
        use crate::*;
        let pool = r2d2_mysql_connection_pool();
        let conn = pool.get().unwrap();
        let row = conn.execute("select 1").unwrap();
        assert_eq!(row, 1);
    }
}