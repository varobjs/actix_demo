use hello_actix::{r2d2_mysql_connection_pool, mysql_connection};
use hello_actix::services::{users_r2d2, users};

use actix_web::{get, Responder, HttpResponse, HttpServer, App, web};
use diesel::MysqlConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use std::sync::Mutex;

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

struct AppState {
    r2d2: Pool<ConnectionManager<MysqlConnection>>,
    mysql: Mutex<MysqlConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL").unwrap();
    let app_data = web::Data::new(AppState {
        r2d2: r2d2_mysql_connection_pool(url.clone()),
        mysql: Mutex::new(mysql_connection(url.clone())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(user1)
            .service(user2)
    })
        .bind("0.0.0.0:7788")?
        .run()
        .await
}
