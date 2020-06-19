
use serde::{Serialize, Deserialize};
use super::attachment::Attachment;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messages {
    pub attachments: Vec<Attachment>,
    pub avatar_url: Option<String>,
    pub created_at: u32,
    pub favorited_by: Vec<String>,
    pub group_id: String,
    pub id: String,
    pub name: String,
    pub sender_id: String,
    pub sender_type: String,
    pub source_guid: String,
    pub system: bool,
    pub text: Option<String>,
    pub user_id: String,
    pub platform: Option<String>,
}
