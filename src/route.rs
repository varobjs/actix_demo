use actix_web::{web, get, Responder, HttpResponse, guard};
use crate::AppState;
use crate::services::users_r2d2;
use crate::controllers::trace;

#[get("/user/1")]
async fn user(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.r2d2;
    let conn = pool.get().unwrap();
    if let Ok(user) = users_r2d2::user_detail(&conn, 1) {
        HttpResponse::Ok().body(format!("query user is {:?}", user))
    } else {
        HttpResponse::Ok().body(format!("cannot found user"))
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // sql trace
            .service(
                web::resource("/trace")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(trace::save_trace))
            )
    )
        .service(user);
}