use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct GuaListReq {
    pub name: String,
    pub limit1: u32,
    pub limit2: u32,
}
