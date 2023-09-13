use warp::Filter;
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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("no id provided"),
        "First Question".to_string(),
        "Content of the question".to_string(),
        Some(vec!("faq".to_string())),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items;
       
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}