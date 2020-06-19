use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub code: u32,
    pub errors: Option<Vec<String>>
}
