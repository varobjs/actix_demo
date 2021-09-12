#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;

pub fn mysql_connection() -> MysqlConnection {
    dotenv::dotenv().ok();
    let mysql_url = std::env::var("DATABASE_URL").unwrap();
    MysqlConnection::establish(&mysql_url).unwrap()
}

mod tests {
    #[test]
    fn test_mysql_connection() {
        use crate::mysql_connection;
        use diesel::prelude::*;

        let conn = mysql_connection();
        let row = conn.execute("select 1").unwrap();
        assert_eq!(row, 1);
    }
}