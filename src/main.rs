extern crate env_logger;

use hello_actix::{r2d2_mysql_connection_pool, mysql_connection};
use hello_actix::services::{users_r2d2, users};

use actix_web::{get, post, Responder, HttpResponse, HttpServer, App, web};
use diesel::MysqlConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use std::sync::Mutex;
use actix_web::middleware::Logger;
use serde::{Serialize, Deserialize};
use hello_actix::models::trace_sqls::NewTraceSqls;
use hello_actix::models::trace_sql_files::NewTraceSqlFiles;

#[derive(Deserialize, Serialize, Debug)]
struct TraceSqlPackage {
    pub app_uuid: Option<String>,
    pub trace_sqls: NewTraceSqlFiles,
    pub trace_files: NewTraceSqlFiles,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/user/1")]
async fn user1(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.r2d2;
    let conn = pool.get().unwrap();
    if let Ok(user) = users_r2d2::user_detail(&conn, 1) {
        HttpResponse::Ok().body(format!("query user is {:?}", user))
    } else {
        HttpResponse::Ok().body(format!("cannot found user"))
    }
}

#[get("/user/2")]
async fn user2(data: web::Data<AppState>) -> impl Responder {
    let conn = data.mysql.lock().unwrap();
    if let Ok(user) = users::user_detail(&conn, 1) {
        HttpResponse::Ok().body(format!("query user is {:?}", user))
    } else {
        HttpResponse::Ok().body(format!("cannot found user"))
    }
}

async fn trace_save(traces: web::Json<TraceSqlPackage>,data: web::Data<AppState>) -> String {
    format!("trace {:?}", traces)
}


struct AppState {
    r2d2: Pool<ConnectionManager<MysqlConnection>>,
    mysql: Mutex<MysqlConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let url = std::env::var("DATABASE_URL").unwrap();
    let app_data = web::Data::new(AppState {
        r2d2: r2d2_mysql_connection_pool(url.clone()),
        mysql: Mutex::new(mysql_connection(url.clone())),
    });

    HttpServer::new(move || {
        App::new()
            .middleware(Logger::default())
            .app_data(app_data.clone())
            .service(index)
            .service(user1)
            .service(user2)
            .service(
                web::resource("/trace")
                    .name("trace_sql")
                    .guard(actix_web::guard::Header("content-type", "application/json"))
                    .route(web::post().to(trace_save))
            )
    })
        .bind("0.0.0.0:7789")?
        .run()
        .await
}
