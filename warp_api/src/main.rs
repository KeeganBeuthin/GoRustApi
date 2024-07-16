mod models;
mod handlers;
mod routes;

use std::sync::Arc;
use tokio::sync::Mutex;
use handlers::Books;

#[tokio::main]
async fn main() {
    let books: Books = Arc::new(Mutex::new(Vec::new()));
    let routes = routes::routes(books);

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}