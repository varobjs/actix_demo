use std::io::Error;
use chrono::NaiveDateTime;
use crate::schema::trace_sqls;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

#[derive(Queryable)]
pub struct TraceSqls {
    pub id: u64,
    pub app_uuid: String,
    pub sql_uuid: String,
    pub db_host: String,
    pub run_host: String,
    pub run_ms: u32,
    pub run_mode: String,
    pub request_uri: String,
    pub referer: String,
    pub trace_sql_md5: String,
    pub trace_sql: String,
    pub trace_sql_binds: String,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable, Validate, Debug, Deserialize, Clone)]
#[table_name = "trace_sqls"]
pub struct NewTraceSqls {
    #[validate(length(equal = 32))]
    pub app_uuid: String,

    #[validate(length(equal = 32))]
    pub sql_uuid: String,

    #[validate(length(max = 32))]
    pub db_host: String,

    #[validate(length(max = 32))]
    pub run_host: String,

    pub run_ms: u32,

    #[validate(length(max = 16))]
    pub run_mode: String,

    #[validate(length(max = 256))]
    pub request_uri: String,

    #[validate(length(max = 256))]
    pub referer: String,

    #[validate(length(equal = 32))]
    pub trace_sql_md5: String,

    #[validate(length(max = 65535))]
    pub trace_sql: String,

    #[validate(length(max = 512))]
    pub trace_sql_binds: String,

    pub created_at: NaiveDateTime,
}

impl NewTraceSqls {
    pub fn from_json(
        value: &mut Value,
        app_uuid: &Option<String>,
    ) -> Result<Self, Error> {
        if let Some(t) = app_uuid {
            value["app_uuid"] = Value::String(t.to_string());
        } else if value.get("app_uuid") == None {
            value["app_uuid"] = Value::String(crate::get_v3_uuid());
        }

        if value.get("sql_uuid") == None {
            value["sql_uuid"] = Value::String(crate::get_v3_uuid());
        }

        let trace: Self = serde_json::from_str(&value.to_string())?;
        Ok(trace)
    }
}