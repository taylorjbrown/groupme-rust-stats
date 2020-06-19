use std::collections::HashMap;
use serde::{Serialize};
use std::fs::File;
use std::io::Write;
use std::error;
use csv::WriterBuilder;
use crate::file_handlers::raw_messages;
use crate::groupme_api_structs::Messages;
use crate::stat_structs::UserStats;



#[derive(Serialize)]
struct Row<'a> {
    pub user_id: &'a str,
    pub user_firstname:&'a String,
    pub user_names: &'a String,
}


pub fn run_stats(user_stats: &mut HashMap<String, UserStats>) -> &mut HashMap<String, UserStats> {

	let messages: Vec<Messages> =
		raw_messages::read_all_in_dir().unwrap();

    for message in messages {
        add_stats(message, user_stats);
	}

	write_users_as_csv(user_stats.clone()).unwrap();

    return user_stats;
}


fn add_stats(
    message: Messages,
    user_stats: &mut HashMap<String, UserStats>,
) {
    let has_user_stats = user_stats.contains_key(&message.user_id);
    let user_id = message.user_id.clone();
    let key = user_id.clone();
    let lookup = key.clone();
    let user_name = message.name.clone();

    let who_liked_my_messages = HashMap::new();
    let whos_messages_did_i_like = HashMap::new();

    let cur_hash: UserStats = UserStats {
        id: user_id,
        first_name: user_name,
        names: vec![message.name].into_iter().collect(),
        avatar_urls: vec![message.avatar_url.unwrap_or_default()]
            .into_iter()
            .collect(),
        total_likes: message.favorited_by.len() as u32,
        total_messages: 1,
        top_like_message: (
            (message.favorited_by.len() as u32),
            message.text.unwrap_or_default(),
        ),
        who_liked_my_messages,
        whos_messages_did_i_like,
        ratio: "0".to_string(),
    };

    if !has_user_stats {
        user_stats.insert(key, cur_hash);
    } else {
        let found_hash = user_stats.get_mut(&lookup).unwrap();
        found_hash.total_likes += cur_hash.total_likes;
        found_hash.total_messages += cur_hash.total_messages;
        found_hash
            .who_liked_my_messages
            .extend(cur_hash.who_liked_my_messages);
        found_hash.names.extend(cur_hash.names);
        found_hash.avatar_urls.extend(cur_hash.avatar_urls);
        if cur_hash.top_like_message.0 > found_hash.top_like_message.0 {
            found_hash.top_like_message = cur_hash.top_like_message
        }

        found_hash.handle_who_liked_my_messages(&message.favorited_by);
        found_hash.ratio = found_hash.calc_likes_vs_messages();
        for fav in message.favorited_by {
            let fav_user_id = fav.clone();
            let &mut user;
            if user_stats.contains_key(&fav) {
                user = user_stats.get(&fav).unwrap().clone();
            } else {
                user = UserStats {
                    id: fav,
                    first_name: "".to_string(),
                    names: vec![].into_iter().collect(),
                    avatar_urls: vec![].into_iter().collect(),
                    total_likes: 0 as u32,
                    total_messages: 1,
                    top_like_message: (0, "".to_string()),
                    who_liked_my_messages: HashMap::new(),
                    whos_messages_did_i_like: HashMap::new(),
                    ratio: "0".to_string(),
                };
            }
            let updated_user = update_like_count(user, &cur_hash.id);
            user_stats.remove(&fav_user_id.clone());
            user_stats.insert(fav_user_id, updated_user);
        }
    }
}

fn update_like_count(mut user: UserStats, message_by_id: &String) -> UserStats {
    if !user.whos_messages_did_i_like.contains_key(message_by_id) {
        user.whos_messages_did_i_like
            .insert(message_by_id.to_string(), 1);
    } else {
        let user_liked_cur = user.whos_messages_did_i_like.get(message_by_id).unwrap();
        let user_liked_old = user_liked_cur.clone() + 1;
        user.whos_messages_did_i_like.remove(message_by_id);
        user.whos_messages_did_i_like
            .insert(message_by_id.to_string(), user_liked_old);
    }
    return user;
}


pub fn write_users_as_csv(user_stats:HashMap<String, UserStats>) -> Result<(),Box<dyn error::Error>> {
    let mut writer = WriterBuilder::new()
         .has_headers(false)
         .from_writer(vec![]);
        
    for user_stat in &user_stats{
        let names: Vec<String> = user_stat.1.names.iter().map(|name| name.clone()).collect();
        let user_names = "{".to_string() + &names.join(", ") + &"}".to_string();

        writer.serialize(Row {
            user_id: &user_stat.1.id,
            user_firstname: &user_stat.1.first_name,
            user_names: &user_names,
        })?;
    }

    let data = String::from_utf8(writer.into_inner()?)?;

    let filetype = ".csv";
    let mut filename = "./results/".to_string();
    filename.push_str("users");
    filename.push_str(&filetype);

    let mut f = File::create(filename).expect("Unable to open file");

    let message_bytes = &data.as_bytes();
    f.write_all(&message_bytes).expect("Unable to write data");

    Ok(())
}