use actix_web::{web, get, guard, Responder, HttpResponse};
use crate::AppState;
use crate::services::trace::{batch_save_traces, RequestNewTraceSql};
use crate::services::users_r2d2;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // trace
            .service(
                web::resource("/trace")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(save_trace))
            )
    )
        .service(index)
        .service(user);
}