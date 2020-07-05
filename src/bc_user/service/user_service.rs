use crate::config::init_db::MysqlPool;
use crate::entity;
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpServer};
use diesel::prelude::*;

// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    uid: i32,
    conn: &MysqlPool,
) -> Result<Option<entity::user_entity::User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let userss = user
        .filter(userid.eq(userid))
        .first::<entity::user_entity::User>(&conn.get().expect(""))
        .optional()?;

    Ok(userss)
}

// Run query using Diesel to insert a new database row and return the result.
/*pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &SqliteConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}*/
