#[cfg(test)]
mod models;
#[cfg(test)]
mod requests;
mod macros;

use std::env;

#[derive(Debug)]
pub(super) struct Configuration {
    login_name: String,
    password: String,
    school_name: String,
    school_id: String,
    article_id: String,
    article_answer: String
}

pub(super) fn setup() -> Configuration {
    let login_name = env::var("LOGIN_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let school_name = env::var("SCHOOL_NAME").unwrap();
    let school_id = env::var("SCHOOL_ID").unwrap();
    let article_id = env::var("ARTICLE_ID").unwrap();
    let article_answer = env::var("ARTICLE_ANSWER").unwrap();
    Configuration {
        login_name,
        password,
        school_name,
        school_id,
        article_id,
        article_answer,
    }
}