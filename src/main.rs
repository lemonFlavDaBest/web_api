use warp::{reject::Reject, Filter, Rejection, Reply, http::StatusCode, http::Method, filters::{cors::CorsForbidden}};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
struct QuestionId(String);

#[derive(Debug, Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Cannot read questions.json")
    }

}

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(err: Rejection) -> Result<impl Reply, Rejection> {
   if let Some(error) = err.find::<CorsForbidden>(){
         Ok(warp::reply::with_status(
              error.to_string(),
              StatusCode::FORBIDDEN,
         ))
   } else {
    Ok(warp::reply::with_status(
        "Route Not Found".to_string(),
        StatusCode::NOT_FOUND,
    ))
   }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_header("content-type");

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_questions.with(cors);
       
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}