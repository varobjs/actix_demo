extern crate env_logger;

use actix_web::{App, http, HttpServer, middleware, web};
use diesel::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use log::info;

use hello_actix::{AppState, error_handler, get_db_pool, MyContext, route};
use hello_actix::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::new();
    info!("Config: {:#?}", config);

    let manager = ConnectionManager::<MysqlConnection>::new(&config.database.url);
    let pool = diesel::r2d2::Pool::builder()
        .max_size(config.database.pool_size)
        .build(manager)
        .unwrap();

    let context = MyContext::create(
        pool,
        config.clone(),
    );

    let url = std::env::var("DATABASE_URL").unwrap();
    let app_data = web::Data::new(AppState {
        r2d2: get_db_pool(url.clone())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            .app_data(app_data.clone())
            .app_data(
                // max size 100kb
                web::JsonConfig::default().limit(102400)
            )
            .wrap(
                middleware::errhandlers::ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, error_handler)
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, error_handler)
                    .handler(http::StatusCode::SERVICE_UNAVAILABLE, error_handler)
            )
            .wrap(middleware::Logger::default())
            .configure(route::config)
    })
        .bind("0.0.0.0:7788")?
        .run()
        .await
}
