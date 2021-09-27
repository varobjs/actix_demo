#[macro_use]
extern crate diesel;

pub mod services;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

/// 创建MySQL连接
pub fn mysql_connection(url: String) -> MysqlConnection {
    MysqlConnection::establish(&url).unwrap()
}

/// 创建MySQL连接池
pub fn r2d2_mysql_connection_pool(url: String) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager: ConnectionManager<MysqlConnection> = diesel::r2d2::ConnectionManager::new(&url);
    diesel::r2d2::Pool::builder()
        .max_size(50)
        .build(manager)
        .unwrap()
}