use actix_web::{get, web, Error,  HttpResponse};

use crate::config::init_db::MysqlPool;
use crate::bc_user::service::user_service;
/// Finds bc_user by UID.
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
        let res = HttpResponse::NotFound().body(format!("No bc_user found with uid: {}", user_uid));
        Ok(res)
    }
}
