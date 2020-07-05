use crate::schema::user;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub userid: i32,
    pub username: String,
}
