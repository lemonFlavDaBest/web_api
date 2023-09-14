use warp::{reject::Reject, Filter, Rejection, Reply, http::StatusCode, http::Method};
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
   if let Some(InvalidId) = err.find() {
       Ok(warp::reply::with_status(
           "Invalid id provided",
           StatusCode::UNPROCESSABLE_ENTITY,
       ))
   } else {
    Ok(warp::reply::with_status(
        "Route Not Found",
        StatusCode::NOT_FOUND,
    ))
   }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_header("content-type");

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);
       
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}