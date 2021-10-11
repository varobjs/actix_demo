extern crate env_logger;

use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::body::ResponseBody;
use actix_web::dev::{ServiceResponse};
use actix_web::error::Result;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use hello_actix::{mysql_connection, r2d2_mysql_connection_pool};
use hello_actix::services::{users, users_r2d2};
use hello_actix::services::trace::batch_save_traces;
use hello_actix::services::trace::RequestNewTraceSql;

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

async fn save_trace(traces: web::Json<Vec<RequestNewTraceSql>>, data: web::Data<AppState>) -> impl Responder {
    let pool = &data.r2d2;
    let conn = pool.get().unwrap();
    let res = match serde_json::to_string(&*traces) {
        Ok(t) => { batch_save_traces(&conn, &t).unwrap() }
        Err(_e) => panic!("error")
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(res)
}


struct AppState {
    r2d2: Pool<ConnectionManager<MysqlConnection>>,
    mysql: Mutex<MysqlConnection>,
}

fn more_handler<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let status = res.status();
    res = res.map_body::<_, B>(|_, _| {
        ResponseBody::Other(
            format!(
                r#"{{"code":{},"message":"{}"}}"#,
                status.as_u16(),
                status.canonical_reason().unwrap_or("")
            )
                .into(),
        )
    });
    res.response_mut()
        .headers_mut()
        .insert(actix_web::http::header::CONTENT_TYPE, actix_web::http::HeaderValue::from_static("application/json"));
    Ok(ErrorHandlerResponse::Response(res))
}

fn route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index)
        .service(user1)
        .service(user2)
        .service(web::resource("/trace")
            .guard(actix_web::guard::Header("content-type", "application/json"))
            .route(web::post().to(save_trace))
        );
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let url = std::env::var("DATABASE_URL").unwrap();
    let app_data = web::Data::new(AppState {
        r2d2: r2d2_mysql_connection_pool(url.clone()),
        mysql: Mutex::new(mysql_connection(url.clone())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(
                web::JsonConfig::default().limit(4096)
            )
            .wrap(
                ErrorHandlers::new()
                    .handler(actix_web::http::StatusCode::NOT_FOUND, more_handler)
                    .handler(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, more_handler)
                    .handler(actix_web::http::StatusCode::SERVICE_UNAVAILABLE, more_handler)
            )
            .wrap(Logger::default())
            .configure(route_config)
    })
        .bind("0.0.0.0:7789")?
        .run()
        .await
}
