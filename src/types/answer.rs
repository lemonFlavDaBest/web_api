use serde::{Deserialize, Serialize};
use crate::types::question::QuestionId;
use std::collections::HashMap;
use warp::http::StatusCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct AnswerId(pub String);