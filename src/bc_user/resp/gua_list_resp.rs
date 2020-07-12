use diesel::sql_types::{Integer, Text};
use serde::{Deserialize, Serialize};
#[derive(QueryableByName, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuaListResp {
    #[sql_type = "Text"]
    gua_code: String,
    #[sql_type = "Text"]
    gua_name: String,
}
