use std::collections::HashMap;
use warp::http::StatusCode;
use crate::types::{answer::{Answer, AnswerId}, question::QuestionId};

pub async fn add_answer(store: crate::store::Store, params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("question_id").unwrap().to_string()),
    };
    store.answers.write().await.insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status("Answer added", StatusCode::OK,))
}