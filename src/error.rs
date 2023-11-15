
    use warp::{filters::{body::BodyDeserializeError, cors::CorsForbidden}, reject::Reject, http::StatusCode, Rejection, Reply};

    #[derive(Debug)]
    pub enum Error {
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

    pub async fn return_error(err: Rejection) -> Result<impl Reply, Rejection> {
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
       } else if let Some(error) = err.find::<BodyDeserializeError>() {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
       } else {
            Ok(warp::reply::with_status(
                "Route Not Found".to_string(),
                StatusCode::NOT_FOUND,
            ))
       }
    }

    impl Reject for Error {}