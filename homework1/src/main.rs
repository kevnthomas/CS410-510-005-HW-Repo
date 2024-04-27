//Kevin Thomas main.rs
//Implementing AXUM version

use axum::{Router, routing::get};
use warp::{http::Method, Filter};

mod error;
mod handlers;
mod faux_db;
mod question;

use crate::handlers::{
    add_questions, delete_question, get_questions, return_error, update_question,
};
use crate::faux_db::Store;

#[tokio::main]
//AXUM Implementation
async fn main(){
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    fn init_router() -> {
        Router::new()
            .route("/", get(get_questions))
    }

}

//Base Implmentation
/*
async fn main() {
    //fake database
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    //allowing reutnr of cors headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    //Warp's chaining of more than one filter via .and to create one big filter and apply to get_items
    //use path::end to signal listening on exactly /question, and not /question/.../
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions)
        .recover(return_error);

    //Allows user to add question
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_questions);

    //Allows user to update question
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    //Allows user to remove question
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    //defines route variables
    let routes = get_items
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    //use path::end to signal listening on exactly /question, and not /question/.../
}
*/