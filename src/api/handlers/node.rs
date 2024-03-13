use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sentence {
    pub sentence: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseData {
    pub word_count: usize,
    pub word_count_unique: usize,
    pub average_word_length: usize,
    pub average_unique_word_length: usize,
    pub char_count: usize,
}

// This is just a simple handler that takes a sentence and returns some statistics about it
// Just practicing string manipulation and basic Rust
pub async fn node(req: web::Json<Sentence>) -> impl Responder {
    let body = req.sentence.as_bytes();
    let sentence = String::from_utf8(body.to_vec()).unwrap();
    let sentence = sentence.to_lowercase();
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let word_count = words.len();
    let word_count_unique = words.iter().collect::<HashSet<_>>().len();
    let char_count = sentence.chars().count();
    let average_word_length = if word_count > 0 {
        char_count / word_count
    } else {
        0
    };
    let average_unique_word_length = if word_count_unique > 0 {
        words.iter().map(|word| word.chars().count()).sum::<usize>() / word_count_unique
    } else {
        0
    };

    HttpResponse::Ok().json(ResponseData {
        word_count,
        word_count_unique,
        average_word_length,
        average_unique_word_length,
        char_count,
    })
}
