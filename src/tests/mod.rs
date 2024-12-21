#[cfg(test)]
mod models;
#[cfg(test)]
mod requests;

use std::env;

#[derive(Debug)]
pub(super) struct Configuration {
    login_name: String,
    password: String,
    school_name: String,
    school_id: String,
    user_id: String, // 理论上user_id值唯一
    class_id: String,
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
    let user_id = env::var("USER_ID").unwrap();
    let class_id = env::var("CLASS_ID").unwrap();
    Configuration {
        login_name,
        password,
        school_name,
        school_id,
        article_id,
        article_answer,
        user_id,
        class_id
    }
}


