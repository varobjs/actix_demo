use std::collections::HashMap;
use std::io::Error;
use std::prelude::rust_2021::FromIterator;

use diesel::{Connection, MysqlConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::trace_sql_files::NewTraceSqlFiles;
use crate::models::trace_sqls::NewTraceSqls;
use crate::schema::trace_sql_files::dsl::trace_sql_files as STraceSqlFiles;
use crate::schema::trace_sqls::dsl::trace_sqls as STraceSqls;

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestNewTraceSqlFile {
    pub app_uuid: Option<String>,
    pub sql_uuid: Option<String>,
    pub trace_file: String,
    pub trace_line: u32,
    pub trace_class: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestNewTraceSql {
    pub app_uuid: Option<String>,
    pub sql_uuid: Option<String>,
    pub db_host: String,
    pub run_host: String,
    pub run_ms: u32,
    pub run_mode: String,
    pub request_uri: String,
    pub referer: String,
    pub trace_sql_md5: String,
    pub trace_sql: String,
    pub trace_sql_binds: String,
    pub created_at: String,
    pub trace_files: Vec<RequestNewTraceSqlFile>,
}

pub fn batch_save_traces(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    data: &str,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut json_data: Value = serde_json::from_str(data)?;
    let json_data = json_data.as_array_mut().expect("格式错误0");
    let mut trace_sqls: Vec<NewTraceSqls> = vec![];
    let mut trace_sql_files: Vec<NewTraceSqlFiles> = vec![];
    let mut return_app_uuids: HashMap<String, _> = HashMap::new();
    let mut return_sql_uuids: HashMap<String, _> = HashMap::new();
    let mut app_uuid = None;
    for json in json_data {
        let trace_sql = NewTraceSqls::from_json(json, &app_uuid).expect("格式错误1");
        app_uuid = Some(trace_sql.app_uuid.clone());
        let sql_uuid = Some(trace_sql.sql_uuid.clone());
        return_app_uuids.insert(trace_sql.app_uuid.clone(), 1);
        return_sql_uuids.insert(trace_sql.sql_uuid.clone(), 1);

        if let Some(trace_files) = json.get_mut("trace_files") {
            if trace_files.is_array() {
                for trace_file in trace_files.as_array_mut().unwrap() {
                    let trace_sql_file =
                        NewTraceSqlFiles::from_json(trace_file, &app_uuid, &sql_uuid).expect("格式错误2");
                    trace_sql_files.push(trace_sql_file.clone());
                }
            }
        }

        trace_sqls.push(trace_sql.clone());
    }

    conn.transaction::<(), diesel::result::Error, _>(|| {
        diesel::insert_into(STraceSqls)
            .values(&trace_sqls)
            .execute(conn)
            .unwrap();

        diesel::insert_into(STraceSqlFiles)
            .values(&trace_sql_files)
            .execute(conn)
            .unwrap();

        Ok(())
    }).ok().unwrap();

    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    res.insert("app_uuid".to_string(), Vec::from_iter(return_app_uuids.keys().map(|t| t.to_string())));
    res.insert("sql_uuid".to_string(), Vec::from_iter(return_sql_uuids.keys().map(|t| t.to_string())));
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        use crate::r2d2_mysql_connection_pool;

        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").unwrap();
        let pool = r2d2_mysql_connection_pool(url);
        let conn = pool.get().unwrap();
        let data = r#"
    [{
            "db_host": "localhost",
            "run_host": "localhost",
            "run_ms": 100,
            "run_mode": "api",
            "pid": 30100,
            "request_uri": "/api/v1/user/info",
            "referer": "",
            "trace_sql_md5": "12345678901234567890123456789021",
            "trace_sql": "select * from users",
            "trace_sql_binds": "",
            "created_at": "2021-10-08T12:00:00.100",
            "trace_files": [{
                "trace_file": "/code/deli.com/1.php",
                "trace_line": 100,
                "trace_class": "DemoController",
                "created_at": "2021-10-08T12:00:00.100"
            },{
                "trace_file": "/code/deli.com/11.php",
                "trace_line": 103,
                "trace_class": "Demo2Controller",
                "created_at": "2021-10-08T12:00:00.100"
            }]
    },{
            "db_host": "localhost",
            "run_host": "localhost",
            "run_ms": 103,
            "run_mode": "api",
            "pid": 3010000,
            "request_uri": "/api/v1/user/info",
            "referer": "",
            "trace_sql_md5": "12345678901234567890123456789021",
            "trace_sql": "select * from users",
            "trace_sql_binds": "",
            "created_at": "2021-10-08T12:00:00.300",
            "trace_files": [{
                "trace_file": "/code/deli.com/2.php",
                "trace_line": 20,
                "trace_class": "TestController",
                "created_at": "2021-10-08T12:00:00.100"
            }]
    }]
    "#;
        let res = batch_save_traces(&conn, data).expect("保存失败");
        println!("{:#?}", res);
    }
}

