use crate::config::init_db::MysqlPool;


use diesel::prelude::*;
use crate::bc_user::entity;
use diesel::sql_query;
use diesel::expression::sql_literal::sql;

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

// Run query using Diesel to insert a new database row and return the result.
/*pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &MysqlPool,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: 2,
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;


    Ok(new_user)
}*/

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
