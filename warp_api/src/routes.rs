use warp::Filter;
use super::handlers;
use super::models::Book;

pub fn routes(books: handlers::Books) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_books(books.clone())
        .or(add_book(books))
}

fn get_books(books: handlers::Books) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("books")
        .and(warp::get())
        .and(with_books(books))
        .and_then(handlers::get_books)
}

fn add_book(books: handlers::Books) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("books")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_books(books))
        .and_then(handlers::add_book)
}

fn with_books(books: handlers::Books) -> impl Filter<Extract = (handlers::Books,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || books.clone())
}