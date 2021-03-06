use crate::config::init_db::MysqlPool;

use diesel::expression::sql_literal::sql;
use diesel::prelude::*;
use diesel::query_builder::SqlQuery;
use diesel::query_dsl::LoadQuery;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

use crate::bc_user::entity;
use crate::bc_user::req::gua_list_req;
use crate::bc_user::resp::gua_list_resp;
use anyhow::{Context, Result};
use crate::utils::CustomeError::CustomerError;
use crate::bc_user::resp::gua_list_resp::GuaListResp;

// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    user_id: i32,
    conn: &MysqlPool,
) -> Result<Option<entity::user_entity::User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let userss = user
        .filter(userid.eq(user_id))
        .first::<entity::user_entity::User>(&conn.get().expect(""))
        .optional()?;

    Ok(userss)
}

pub fn search_gua_list(
    req: &gua_list_req::GuaListReq,
    conn: &MysqlPool,
) -> Result<Vec<GuaListResp>,CustomerError> {
    let param = &req.name;
    info!("sql查询参数{:#?}", param);
    let query = sql_query("select gua_code,gua_name from six_four_hexagrams where gua_code=?");
    let results = query
        .bind::<Text, _>(param)
        .get_results::<gua_list_resp::GuaListResp>(&conn.get().expect("数据库连接异常"));
    info!("sql查询参数{:#?}", results);
  if let Ok(results)=results{
        Ok(results)
    }else{
      let errorMsg = String::from("sql执行异常");
        Err(CustomerError::SqlExecutionError(errorMsg))
    }

    //println!("{:#?}", results);

}
