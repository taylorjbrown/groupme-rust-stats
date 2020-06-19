use indexmap::IndexSet;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt;

type Id = String;

#[derive(Debug, Clone, Eq)]
pub struct UserStats {
    pub id: String,
    pub first_name: String,
    pub names: IndexSet<String>,
    pub avatar_urls: HashSet<String>,
    pub total_likes: u32,
    pub total_messages: u32,
    pub top_like_message: (u32, String),
    pub who_liked_my_messages: HashMap<String, u32>,
    pub whos_messages_did_i_like: HashMap<String, u32>,
    pub ratio: String,
}

impl UserStats {
    pub fn calc_likes_vs_messages(&self) -> String {
        return (self.total_likes as f64 / self.total_messages as f64).to_string();
    }
    pub fn handle_who_liked_my_messages(&mut self, favorited_by: &Vec<String>) {
        for favorited in favorited_by {
            if !self.who_liked_my_messages.contains_key(favorited) {
                self.who_liked_my_messages.insert(favorited.clone(), 1);
            } else {
                let user_liked_cur = self.who_liked_my_messages.get(favorited).unwrap();
                let user_liked_old = user_liked_cur.clone() + 1;
                self.who_liked_my_messages.remove(favorited);
                self.who_liked_my_messages
                    .insert(favorited.clone(), user_liked_old);
            }
        }
    }
}

impl PartialEq for UserStats {
    fn eq(&self, other: &UserStats) -> bool {
        return self.id == other.id;
    }
}

impl Hash for UserStats {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<Id> for UserStats {
    fn borrow(&self) -> &Id {
        &self.id
    }
}

impl fmt::Display for UserStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "User Name: {:?}
        \nNames:\n{:?}
        \nAvatars:\n{:?}
        \nTotal Likes: {:?}
        \nTop Liked Messages:\n {:?}
        \nRatio: {:?}", 
        self.first_name,
        self.names, 
        self.avatar_urls, 
        self.total_likes, 
        self.top_like_message, 
        self.ratio);
    }  
}