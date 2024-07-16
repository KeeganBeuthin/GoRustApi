use warp::Reply;
use super::models::Book;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Books = Arc<Mutex<Vec<Book>>>;

pub async fn get_books(books: Books) -> Result<impl Reply, warp::Rejection> {
    let books = books.lock().await;
    Ok(warp::reply::json(&*books))
}

pub async fn add_book(new_book: Book, books: Books) -> Result<impl Reply, warp::Rejection> {
    let mut books = books.lock().await;
    books.push(new_book);
    Ok(warp::reply::with_status(
        "Book added",
        warp::http::StatusCode::CREATED,
    ))
}