use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct TraceSqlCode {
    pub id: u64,
    pub app_uuid: String,
    pub sql_uuid: String,
    pub file_id: u64,
    pub source_code: String,
    pub created_at: NaiveDateTime,
}