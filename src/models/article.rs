use crate::config::DATE_FORMAT;
use crate::models::user::User;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: i32,
    pub tag_list: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub favorites_count: i32,
}

impl Article {
    pub fn attach(self, author: User, favorited: bool) -> ArticleJson {
        ArticleJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            author,
            tag_list: self.tag_list,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
            favorites_count: self.favorites_count,
            favorited,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleJson {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: User,
    pub tag_list: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorites_count: i32,
    pub favorited: bool,
}
