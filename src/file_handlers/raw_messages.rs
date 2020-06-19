use crate::groupme_api_structs::Messages;
use crate::groupme_api_structs::Data;
use rayon::prelude::*;
use config;
use glob::glob;
use std::fs::remove_dir_all;
use std::fs::create_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error;
use csv::WriterBuilder;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};
use serde::{Serialize};

#[derive(Serialize)]
struct Row<'a> {
    pub created_at: &'a String,
	pub user_id: &'a str,
	pub name: &'a str,
    pub text_val:&'a Option<String>,
	pub favorited_by: &'a str,
	pub totaldabs: &'a usize,
    pub message_id: &'a str,
}



pub fn clear_folder(folder:&String){
    if Path::new(folder).exists() {
        remove_dir_all(folder.clone()).unwrap_or_default();
    }
    create_dir(folder).unwrap_or_default();
}

pub fn create_file(data: &Data, output_folder:&String) {
    let filename = create_filename(data, output_folder);

    let mut f = File::create(filename).expect("Unable to open file");

    let messages_serial = serde_json::to_string(&data.messages).unwrap();
    let message_bytes = &messages_serial.as_bytes();
    f.write_all(&message_bytes).expect("Unable to write data");
}

pub fn create_filename(data: &Data, output_folder: &String) -> String{

    let mut filename = output_folder.to_string();
    let delimiter = "-";
    let filetype = ".json";
    filename.push_str(&data.start_timestamp.to_string());
    filename.push_str(&delimiter);
    filename.push_str(&data.next_start_timestamp.to_string());
    filename.push_str(&filetype);
    return filename;
}

pub fn read_all_in_dir() -> Result<Vec<Messages>, ()> {
	let mut settings = config::Config::default();
    settings
		.merge(config::File::with_name("Settings")).unwrap()
		.merge(config::Environment::with_prefix("APP")).unwrap();
    let output_folder =  settings.get_str("output_folder")
										.unwrap_or_default();

    let directory:&str = &(output_folder.to_owned() + "/*.json");
    let mut files: Vec<String> = Vec::new();
    for entry in glob(directory).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) =>  { 
                println!("{:?}", path.display());
                let path = path.into_os_string().into_string().unwrap();
    
                files.push(path);
            },  
            Err(e) => println!("{:?}", e),
        }
    }

    let par_iter = files
        .par_iter()
        .map(|path_string| return read_file_to_json(path_string))
        .flatten();
        
    let messages :Vec<Messages> = par_iter.collect();

    write_messages_as_csv(messages.clone()).unwrap();

    Ok(messages)
}


pub fn read_file_to_json(filename:&str) -> Vec<Messages>{
    let file = File::open(filename.to_string())
        .expect("file should open read only");

    let messages: Vec<Messages> = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    return messages;
}

pub fn write_messages_as_csv(messages:Vec<Messages>) -> Result<(), Box<dyn error::Error>> {
    let mut writer = WriterBuilder::new()
         .has_headers(false)
         .from_writer(vec![]);

    for message in &messages{
        
        let favorited_by = "{".to_string() + &message.favorited_by.clone().join(", ") + &"}".to_string();

        writer.serialize(Row {
            created_at: &convert_to_datetime(message.created_at),
            user_id: &message.user_id,
			text_val: &message.text,
			totaldabs: &message.favorited_by.len(),
			name: &message.name,
            favorited_by: &favorited_by,
            message_id: &message.id,
        })?;
    }

    let data = String::from_utf8(writer.into_inner()?)?;

    let filetype = ".csv";
    let mut filename = "./results/".to_string();
    filename.push_str("messages");
    filename.push_str(&filetype);

    let mut f = File::create(filename).expect("Unable to open file");

    let message_bytes = &data.as_bytes();
	f.write_all(&message_bytes).expect("Unable to write data");
	

    Ok(())
}


fn convert_to_datetime(unixtime: u32) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(unixtime.into());
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    return timestamp_str
}
