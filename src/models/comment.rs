use crate::config::DATE_FORMAT;
use crate::models::user::User;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub article: i32,
    pub author: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comment {
    pub fn attach(self, author: User) -> CommentJson {
        CommentJson {
            id: self.id,
            body: self.body,
            author,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentJson {
    pub id: i32,
    pub body: String,
    pub author: User,
    pub created_at: String,
    pub updated_at: String,
}
