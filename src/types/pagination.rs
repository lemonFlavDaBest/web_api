use std::collections::HashMap;
use crate::error;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, error::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Ok(Pagination {
            start: params.get("start").unwrap().parse::<usize>().expect("could not parse start"),
            end: params.get("end").unwrap().parse::<usize>().expect("could not parse end"),
        })
    } else {
        Err(error::Error::MissingParameters)
    }
}