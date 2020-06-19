use serde::{Deserialize};
use super::message::Messages;

#[derive(Deserialize, Debug, Clone)]
pub struct DataResponse {
    pub count: u32,
    pub messages: Option<Vec<Messages>>,
}
