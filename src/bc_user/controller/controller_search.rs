use actix_web::{get, post, web, Error, HttpResponse};

use crate::bc_user::entity::user_entity;
use crate::bc_user::req::gua_list_req;
use crate::bc_user::service::user_service;
use crate::config::init_db::MysqlPool;
use crate::utils::ResultMsg::ApiResult;
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

#[post("/guasearch")]
pub async fn get_gua_list(
    pool: web::Data<MysqlPool>,
    form: web::Json<gua_list_req::GuaListReq>,
) -> Result<HttpResponse, Error> {
 /*   error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");*/
    let conn = pool.get().expect("couldn't get db connection from pool");
    let req = form.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    let result = web::block(move || user_service::search_gua_list(&req, &pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    info!("33333");
    Ok(HttpResponse::Ok().json(ApiResult::new().with_msg("success").with_data(result)))
}
