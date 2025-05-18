use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub title: String,
    pub filename: String,
    pub date: NaiveDateTime,
    pub latest: Option<NaiveDateTime>,
    pub published: Option<bool>,
    pub tags: Vec<String>,
    pub category: String,
    pub description: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub title: String,
    pub filename: String,
    pub content: Option<String>,
}