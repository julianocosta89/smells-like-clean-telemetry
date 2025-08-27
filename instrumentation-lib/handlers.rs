use actix_web::{Responder, get, web};

use crate::db;

#[get("/songs/{title}/{artist}")]
pub async fn songs(path: web::Path<(String, String)>) -> impl Responder {
    let (title, artist) = path.into_inner();

    // First check if song already exists
    match db::find_by_title_and_artist(&title.to_lowercase(), &artist.to_lowercase()).await {
        Ok(result) => return result,
        Err(error) => error,
    }
}
