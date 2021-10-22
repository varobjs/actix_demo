#[macro_use]
extern crate diesel;

use actix_web::dev::{ResponseBody, ServiceResponse};
use actix_web::error::Result;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use crate::config::Config;

pub mod services;
pub mod models;
pub mod schema;
pub mod route;
pub mod controllers;
pub mod helpers;
pub mod config;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

#[derive(Clone)]
pub struct MyContext {
    pub pool: DbPool,
    pub config: Config,
}

impl MyContext {
    pub fn create(
        pool: DbPool,
        config: Config,
    ) -> MyContext {
        MyContext {
            pool,
            config,
        }
    }
}

pub struct AppState {
    pub r2d2: DbPool,
}

pub fn get_db_pool(url: String) -> DbPool {
    let manager: diesel::r2d2::ConnectionManager<diesel::MysqlConnection>
        = diesel::r2d2::ConnectionManager::new(&url);
    diesel::r2d2::Pool::builder()
        .max_size(50)
        .build(manager)
        .unwrap()
}

pub fn gen_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string().replace("-", "")
}

pub fn error_handler<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
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
        .insert(
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::HeaderValue::from_static("application/json"),
        );
    Ok(ErrorHandlerResponse::Response(res))
}