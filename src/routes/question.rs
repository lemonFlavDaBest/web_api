use crate::store;
use crate::types::{question::{Question, QuestionId}, pagination::{Pagination, extract_pagination}};
use crate::error;
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn get_questions(params: HashMap<String, String>, store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(store: store::Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
    store.questions.write().await.insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::CREATED,))
}

pub async fn update_question(id: String, store: store::Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => {
            *q = question;
            Ok(warp::reply::with_status("Question updated", StatusCode::OK,))
        },
        None => Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }
}

pub async fn delete_question(id: String, store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK,)),
        None => Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }
}