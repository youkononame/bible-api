use rocket::serde::{Serialize, Deserialize};
use std::sync::LazyLock;
use serde_json::from_str;

#[derive(Clone, Serialize, Deserialize)]
pub struct Verse {
    pub verse: i32,
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter: i32,
    pub verses: Vec<Verse>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bible {
    pub translation: String,
    pub books: Vec<Book>,
}

pub static BIBLE: LazyLock<Bible> = std::sync::LazyLock::new(|| from_str(&include_str!("bible.json")).unwrap());