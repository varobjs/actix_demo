table! {
    trace_sql_code (id) {
        id -> Unsigned<Bigint>,
        app_uuid -> Char,
        sql_uuid -> Char,
        file_id -> Unsigned<Bigint>,
        source_code -> Text,
        created_at -> Datetime,
    }
}

table! {
    trace_sql_files (id) {
        id -> Unsigned<Bigint>,
        app_uuid -> Char,
        sql_uuid -> Char,
        trace_file -> Varchar,
        trace_line -> Unsigned<Integer>,
        trace_class -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    trace_sqls (id) {
        id -> Unsigned<Bigint>,
        app_uuid -> Char,
        sql_uuid -> Char,
        db_host -> Varchar,
        run_host -> Varchar,
        run_ms -> Unsigned<Integer>,
        pid -> Unsigned<Integer>,
        run_mode -> Varchar,
        request_uri -> Varchar,
        referer -> Varchar,
        trace_sql_md5 -> Char,
        trace_sql -> Text,
        trace_sql_binds -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        is_deleted -> Tinyint,
    }
}

allow_tables_to_appear_in_same_query!(
    trace_sql_code,
    trace_sql_files,
    trace_sqls,
    users,
);
