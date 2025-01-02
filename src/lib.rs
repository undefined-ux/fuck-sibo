pub mod cli;
mod error;
mod model;
pub mod prelude;
#[cfg(test)]
mod tests;
mod utils;

use chrono::{Local, Timelike};
use model::*;
use prelude::*;
use utils::request;
pub async fn login(
    user_name: &str,
    password: &str,
    school_id: &str,
) -> SiboResult<UserInformation> {
    let data = LoginRequestBodyParamBuilder::default()
        .login_name(user_name.to_string())
        .password(password.to_string())
        .school_id(school_id.to_string())
        .build()
        .unwrap();

    let resp: LoginResponse = request(JyhCode::Login, Box::new(data)).await?;
    Ok(Into::<UserInformation>::into(resp))
}

pub async fn search_school(keyword: &str) -> SiboResult<Vec<SchoolInformation>> {
    let data: GetSchoolInformationParam = GetSchoolInformationParamBuilder::default()
        .keyword(keyword.to_string())
        .build()
        .unwrap();
    let resp: GetSchoolIDResponse =
        request(JyhCode::SearchSchoolInformation, Box::new(data)).await?;
    Ok(resp.into_iter().map(SchoolInformation::from).collect())
}

pub async fn get_classes(user_id: &str) -> SiboResult<Vec<ClassInformation>> {
    let data = GetClassInformationParamBuilder::default()
        .user_id(user_id.to_string())
        .build()
        .unwrap();
    let resp: GetStudentClassIDResponse =
        request(JyhCode::GetClassInformation, Box::new(data)).await?;
    Ok(resp.into_iter().map(ClassInformation::from).collect())
}

async fn get_article_question(article_id: &str) -> SiboResult<Vec<ArticleQuestion>> {
    let param = GetArticlesQuestionsParamBuilder::default()
        .essay_id(article_id.to_string())
        .build()
        .unwrap();

    let resp: GetArticlesQuestionsResponse =
        request(JyhCode::GetArticleQuestions, Box::new(param)).await?;

    Ok(resp.into_iter().map(ArticleQuestion::from).collect())
}

pub async fn get_article_questions(mut article: Article) -> SiboResult<Article> {
    if article.questions.is_some() {
        return Ok(article);
    }
    let questions = get_article_question(&article.id.clone()).await?;
    article.fill_questions(questions);
    Ok(article)
}

pub async fn get_articles(
    user_id: &str,
    class_id: &str,
    page_size: Option<i32>,
    crawl_questions: Option<bool>,
) -> SiboResult<Vec<Article>> {
    let crawl_questions = crawl_questions.unwrap_or(false);
    let page_size = match page_size {
        Some(x) => {
            if x <= 0 {
                return Err(SiboError::UnknownError {
                    message: "Page size must be greater than 0".to_string(),
                });
            } else {
                x
            }
        }
        None => i32::MAX,
    };
    let data: GetArticlesParam = GetArticlesParamBuilder::default()
        .user_id(user_id.to_string())
        .class_id(class_id.to_string())
        .page_size(page_size)
        .build()
        .unwrap();
    let resp: GetArticlesResponse = request(JyhCode::GetArticles, Box::new(data)).await?;

    if !crawl_questions {
        return Ok(resp
            .into_iter()
            .map(|res| Article::new(res, None))
            .collect());
    }

    let mut result: Vec<Article> = Vec::new();
    for article in resp {
        let questions = get_article_question(&article.essay_id).await?;
        let res = Article::new(article, Some(questions));
        result.push(res);
    }

    Ok(result)
}

pub async fn submit_article(
    user_id: &str,
    class_id: &str,
    article: &Article,
    submit_date_time: Option<&str>,
) -> SiboResult<()> {
    let mut article = article.clone();
    let mut submit_datetime = submit_date_time.map(String::from);
    if article.questions.is_none() {
        article = get_article_questions(article).await?
    }
    if submit_datetime.is_none() {
        let time = Local::now().with_nanosecond(0).unwrap();
        submit_datetime = Some(time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    }

    let data = SubmitArticlesParamBuilder::default()
        .essay_id(article.id.clone())
        .user_id(user_id.to_string())
        .class_id(class_id.to_string())
        .create_time(submit_datetime.unwrap())
        .answer(article.answer.unwrap())
        .build()
        .unwrap();

    match request::<String>(JyhCode::SubmitArticle, Box::new(data)).await {
        Ok(_) => Ok(()),
        Err(err) => match err {
            SiboError::RequestFailed { error_message, .. } => {
                if error_message == "Empty Data field" {
                    Ok(())
                } else {
                    Err(SiboError::SubmitFailed {
                        message: error_message,
                    })
                }
            }
            _ => unreachable!("{:?}", err),
        },
    }
}
