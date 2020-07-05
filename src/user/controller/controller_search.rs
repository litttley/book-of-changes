use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};

use crate::config::init_db::MysqlPool;
use crate::service::user_service;
/// Finds user by UID.
#[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<MysqlPool>,
    user_uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || user_service::find_user_by_uid(user_uid, &pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}
