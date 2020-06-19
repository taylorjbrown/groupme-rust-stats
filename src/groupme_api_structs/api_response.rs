use serde::{Deserialize};
use super::data_response::DataResponse;
use super::meta::Meta;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub response: DataResponse,
    pub meta: Meta,
}