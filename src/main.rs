use warp::{reject::Reject, Filter, Rejection, Reply, http::StatusCode, http::Method, filters::{cors::CorsForbidden}};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
struct QuestionId(String);

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self {
            Error::ParseError(e) => write!(f, "Parse Error: {}", e),
            Error::MissingParameters => write!(f, "Missing Parameters"),
            Error::QuestionNotFound => write!(f, "Question Not Found"),
        }
    }
}

impl Reject for Error {}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Store::init())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Cannot read questions.json")
    }

}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Ok(Pagination {
            start: params.get("start").unwrap().parse::<usize>().expect("could not parse start"),
            end: params.get("end").unwrap().parse::<usize>().expect("could not parse end"),
        })
    } else {
        Err(Error::MissingParameters)
    }
}

async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
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

async fn add_question(store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
    store.questions.write().await.insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::CREATED,))
}

async fn update_question(id: String, store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => {
            *q = question;
            Ok(warp::reply::with_status("Question updated", StatusCode::OK,))
        },
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn return_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = err.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = err.find::<CorsForbidden>(){
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
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .with(cors)
        .recover(return_error);
       
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}