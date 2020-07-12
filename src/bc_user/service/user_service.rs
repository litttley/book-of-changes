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
) -> Result<Option<Vec<gua_list_resp::GuaListResp>>, diesel::result::Error> {
    let param = &req.name;
    println!("{}", param);
    let query = sql_query("select gua_code,gua_name from six_four_hexagrams where gua_code=?");
    let results = query
        .bind::<Text, _>(param)
        .get_results::<gua_list_resp::GuaListResp>(&conn.get().expect("sssccc"));
    println!("{:#?}", results);
    Ok(results.ok())
}

//Using raw sql to get structs not related to any table schema
/*pub fn query_no_schema(){
    #[derive(QueryableByName)]
    struct Entry {
        #[sql_type = "Integer"]
        record_type: i32,
        #[sql_type = "Numeric"]
        value: f64,
    }

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url).unwrap();

        let query = "SELECT record_type, sum(price * amount) as value from records";
        let results = sql_query(query).load::<Entry>(&connection);

}*/
