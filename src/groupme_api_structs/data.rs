use super::message::Messages;

pub struct Data {
    pub total_count: u32,
    pub start_timestamp: u32,
    pub next_start_timestamp: u32,
    pub last_message_id:String,
    pub messages: Vec<Messages>,
}