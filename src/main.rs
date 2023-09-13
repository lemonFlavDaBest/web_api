use warp::{reject::Reject, Filter, Rejection, Reply, http::StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of the question".to_string(),
        Some(vec!("faq".to_string())),
    );

    match question.id.0.parse::<i32>() {
        Ok(_) => Ok(warp::reply::json(&question)),
        Err(_) => return Err(warp::reject::custom(InvalidId)),
    }
}

async fn return_error(err: Rejection) -> Result<impl Reply, Rejection> {
   
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;
       
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}