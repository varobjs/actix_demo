extern crate env_logger;

use actix_web::{App, HttpServer, web};
use actix_web::body::ResponseBody;
use actix_web::dev::ServiceResponse;
use actix_web::error::Result;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;

use hello_actix::{AppState, r2d2_mysql_connection_pool};
use hello_actix::route::config;

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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let url = std::env::var("DATABASE_URL").unwrap();
    let app_data = web::Data::new(AppState {
        r2d2: r2d2_mysql_connection_pool(url.clone())
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
            .configure(config)
    })
        .bind("0.0.0.0:7788")?
        .run()
        .await
}
