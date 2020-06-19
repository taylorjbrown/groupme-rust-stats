use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub r#type:String,
    pub url: Option<String>,
    pub lat: Option<String>,
    pub lng: Option<String>,
    pub name: Option<String>,
    pub token:  Option<String>,
    pub placeholder: Option<String>,
    pub charmap: Option<Vec<Vec<u32>>>,   
}
