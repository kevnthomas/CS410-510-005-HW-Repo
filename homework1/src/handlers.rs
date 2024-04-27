use std::collections::HashMap;
use warp::{filters::cors::CorsForbidden, http::StatusCode, reject::Reject, Rejection, Reply};

use crate::error::Error;
use crate::faux_db::Store;
use crate::question::{Question, QuestionId};

#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}

//route handler for create questions
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    //create question
    /*
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of Question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => {
            //use warp's json reply to return a json version of the question
            Ok(warp::reply::json(&question))
        }
    }
    */

    /*
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> =
            store.questions
                .read()
                .await
                .values()
                .cloned()
                .collect();
        let res = &res[pagination.start..pagination.end];

        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> =
            store.questions
                .read()
                .await
                .values()
                .cloned()
                .collect();

        Ok(warp::reply::json(&res))
    }
    */

    let mut start = 0;

    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Could not parse start");
    }

    println!("{}", start);

    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

    Ok(warp::reply::json(&res))
}

pub async fn add_questions(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

//update question
pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

//remove question
pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

//route handler for return error
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
