use crate::file_handlers::formatted_results;
use crate::stat_structs::UserStats;
use std::collections::HashMap;


pub fn formate_result(results: &mut HashMap<std::string::String, UserStats>) -> Vec<Vec<String>> {
  let mut lookup = HashMap::new();
  lookup.extend(results.into_iter().map(|(k, v)| (k.clone(), v.clone())));
  let mut file_results:Vec<Vec<String>> = Vec::new();

  for (_user_id, stats) in results {
    let current_user = &mut Vec::new();

    let mut who_liked_my_messages: Vec<(u32, String)> = stats
      .who_liked_my_messages
      .iter()
      .map(|(k, v)| (v.clone(), k.clone()))
      .collect();
    who_liked_my_messages.sort_by(|a, b| b.cmp(a));

    let mut whos_messages_did_i_like: Vec<(u32, String)> = stats
      .whos_messages_did_i_like
      .iter()
      .map(|(k, v)| (v.clone(), k.clone()))
      .collect();
    whos_messages_did_i_like.sort_by(|a, b| b.cmp(a));

    current_user.push(stats.to_string());
    current_user.push("\nWho Liked My Messages".to_string());
    for (count, liked_by) in who_liked_my_messages {
      current_user.push(format!(
        "\n({:?}:{:?})",
        lookup.get(&liked_by).unwrap().first_name,
        count
      ));
    }
    current_user.push("\n".to_string());
    current_user.push("\nWhos Messages Did I Like".to_string());
    for (count, liked_by) in whos_messages_did_i_like {
      current_user.push(format!(
        "\n({:?}:{:?})",
        lookup.get(&liked_by).unwrap().first_name,
        count
      ));
    }
    current_user.push("\n____________________________________________________________________________________\n".to_string());
    file_results.push(current_user.clone());
  }
  file_results
}

pub fn save_result(results: &mut HashMap<std::string::String, UserStats>) {
  let results = formate_result(results);
  formatted_results::save_as_text(results.clone());
}
