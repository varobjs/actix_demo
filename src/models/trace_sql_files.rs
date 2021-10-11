use std::io::Error;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::value::Value::Null;
use validator::Validate;

use crate::schema::trace_sql_files;

#[derive(Queryable)]
pub struct TraceSqlFiles {
    pub id: u64,
    pub app_uuid: String,
    pub sql_uuid: String,
    pub trace_file: String,
    pub trace_line: u32,
    pub trace_class: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Validate, Deserialize, Serialize, Clone)]
#[table_name = "trace_sql_files"]
pub struct NewTraceSqlFiles {
    #[validate(length(equal = 32))]
    pub app_uuid: String,

    #[validate(length(equal = 32))]
    pub sql_uuid: String,

    #[validate(length(max = 128))]
    pub trace_file: String,

    pub trace_line: u32,

    #[validate(length(max = 128))]
    pub trace_class: String,

    pub created_at: NaiveDateTime,
}

impl NewTraceSqlFiles {
    pub fn from_json(
        value: &mut Value,
        app_uuid: &Option<String>,
        sql_uuid: &Option<String>,
    ) -> Result<Self, Error> {
        if let Some(t) = app_uuid {
            value["app_uuid"] = Value::String(t.to_string());
        } else if value.get("app_uuid") == None || value.get("app_uuid") == Some(&Null) {
            value["app_uuid"] = Value::String(crate::get_v3_uuid());
        }

        if let Some(t) = sql_uuid {
            value["sql_uuid"] = Value::String(t.to_string());
        } else if value.get("sql_uuid") == None || value.get("sql_uuid") == Some(&Null) {
            value["sql_uuid"] = Value::String(crate::get_v3_uuid());
        }

        let trace: Self = serde_json::from_str(&value.to_string())?;
        Ok(trace)
    }
}