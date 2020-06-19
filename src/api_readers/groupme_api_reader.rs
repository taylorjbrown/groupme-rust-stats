use reqwest::{Client, Error};
use crate::groupme_api_structs::*;
use crate::file_handlers::raw_messages::create_file;
use crate::file_handlers::raw_messages::clear_folder;


#[derive(Debug)]
pub struct MessagePages {
    start_timestamp: u32,
    next_start_timestamp: u32,
    last_message_id: String,
    messages: <Vec<Messages> as IntoIterator>::IntoIter,
    client: reqwest::Client,
    per_page: u32,
    cur_count:u32,
    total_count: u32,
    group_id: u32,
    api_key:String,
    output_folder:String,
}

impl MessagePages {
    
    pub fn of(api_key:&String, group_id: &u32, output_folder: &String) -> Result<Self, Error> {
        Ok(MessagePages {
               group_id: group_id.to_owned(),
               next_start_timestamp: 0,
               messages: vec![].into_iter(),
               client: Client::new(),
               per_page: 100,
               cur_count:0,
               start_timestamp:0,
               total_count: 0,
               last_message_id: "".to_string(),
               api_key:api_key.to_string(),
               output_folder: output_folder.to_string(),
           })
    }

    fn try_next(&mut self) -> Result<Option<Messages>, Error> {

        if let Some(dep) = self.messages.next() {
            return Ok(Some(dep));
        }

        if self.cur_count > 0 && self.cur_count >= self.total_count {
            return Ok(None);
        }

        let mut url = format!("https://api.groupme.com/v3/groups/{}/messages?before_id={}&limit={}",
                          self.group_id,
                          self.last_message_id,
                          self.per_page);
        if self.cur_count == 0 {
            url = format!("https://api.groupme.com/v3/groups/{}/messages?limit={}",
                          self.group_id,
                          self.per_page);
        }
        println!("{:?}", url);
        let mut resp = self.client.get(&url)
                .header("X-Access-Token", self.api_key.clone())
                .send()?;
        
        if resp.status() != 200 {
            return Ok(None);
        }
    
        let mut buf:Vec<u8> = vec![];
        resp.copy_to(&mut buf)?;

        let data = get_data(&buf);
        create_file(&data, &self.output_folder);

        self.cur_count += data.messages.len() as u32;
        self.start_timestamp = data.start_timestamp;
        self.next_start_timestamp = data.next_start_timestamp;
        self.last_message_id = data.last_message_id;
        self.messages = data.messages.into_iter();
        self.total_count = data.total_count;
        
        Ok(self.messages.next())
    }
}



fn get_data(buf:&Vec<u8>)-> Data {
    let data  = serde_json::from_slice::<ApiResponse>(buf).unwrap();
    let total_count = data.response.count;
    let messages =  data.response.messages.unwrap();

    let first_message = messages.first();
    let last_message = messages.last();

    let last_message_id = &last_message.unwrap().id;
    let start_timestamp = first_message.unwrap().created_at;
    let next_start_timestamp = last_message.unwrap().created_at;
    return Data {
        total_count, 
        start_timestamp,
        next_start_timestamp,
        last_message_id: last_message_id.to_string(),
        messages: messages};
}


impl Iterator for MessagePages {
    type Item = Result<Messages, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}


pub fn run_reader(api_key:&String, group_id:u32, output_folder:&String) -> Result<(), Error> {

    clear_folder(output_folder);
    for _message in MessagePages::of(api_key, &group_id, output_folder)? {
    }

    Ok(())
}