use crate::models;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> Json<models::Bible> {
    Json(models::BIBLE.clone())
}

#[get("/book/<book_name>")]
pub fn get_book(book_name: String) -> Result<Json<models::Book>, status::NotFound<String>> {
    for book in &models::BIBLE.books {
        if book.name == book_name {
            return Ok(Json(book.clone()));
        }
    }

    Err(status::NotFound(format!("Book '{}' not found", book_name)))
}

#[get("/book/<book_name>/chapter/<chapter_number>")]
pub fn get_chapter(book_name: String, chapter_number: usize) -> Result<Json<models::Chapter>, status::NotFound<String>> {
    for book in &models::BIBLE.books {
        if book.name != book_name {
            continue;
        }

        if let Some(chapter) = book.clone().chapters.get(chapter_number) {
            let chapter: models::Chapter = chapter.clone();
            return Ok(Json(chapter));
        } else {
            break;
        }
    }

    Err(status::NotFound(format!("Book '{}' chapter {} not found", book_name, chapter_number)))
}

#[get("/book/<book_name>/chapter/<chapter_number>/verse/<verse_number>")]
pub fn get_verse(book_name: String, chapter_number: usize, verse_number: usize) -> Result<Json<models::Verse>, status::NotFound<String>> {
    for book in &models::BIBLE.books {
        if book.name != book_name {
            continue;
        }

        let chapter = if let Some(chapter) = book.clone().chapters.get(chapter_number - 1) {
            chapter.clone()
        } else {
            break;
        };
        
        if let Some(verse) = chapter.verses.get(verse_number) {
            let verse: models::Verse = verse.clone();
            return Ok(Json(verse));
        } else {
            break;
        }
    }

    Err(status::NotFound(format!("Citation '{} {}:{}' not found", book_name, chapter_number, verse_number)))
}