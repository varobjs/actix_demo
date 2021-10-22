use actix_web::{HttpResponse, Responder, web};
use crate::AppState;
use crate::services::trace::{batch_save_traces, RequestNewTraceSql};

pub(crate) async fn save_trace(
    traces: web::Json<Vec<RequestNewTraceSql>>,
    data: web::Data<AppState>,
) -> impl Responder {
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