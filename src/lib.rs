mod error;
mod model;
pub mod prelude;
mod utils;

#[cfg(test)]
mod tests;

use model::*;
use prelude::*;
use utils::request;

pub async fn login(user_name: &str, password: &str, school_id: &str) -> Result<UserInformation> {
    let data = LoginRequestBodyParamBuilder::default()
        .login_name(user_name.to_string())
        .password(password.to_string())
        .school_id(school_id.to_string())
        .build().unwrap();

    let resp: LoginResponse = request::<LoginResponse>(JyhCode::Login, Box::new(data)).await?;
    Ok(Into::<UserInformation>::into(resp))
}

pub async fn search_school(keyword: &str) -> Result<Vec<SchoolInformation>> {
    let data: GetSchoolInformationParam = GetSchoolInformationParamBuilder::default()
        .keyword(keyword.to_string())
        .build().unwrap();
    let resp: GetSchoolIDResponse =
        request::<GetSchoolIDResponse>(JyhCode::SearchSchoolInformation, Box::new(data)).await?;
    Ok(resp.into_iter().map(SchoolInformation::from).collect())
}

pub async fn get_classes(user_id: &str) -> Result<Vec<ClassInformation>> {
    let data = GetClassInformationParamBuilder::default()
        .user_id(user_id.to_string())
        .build()
        .unwrap();
    let resp: GetStudentClassIDResponse =
        request::<GetStudentClassIDResponse>(JyhCode::GetClassInformation, Box::new(data)).await?;
    Ok(resp.into_iter().map(ClassInformation::from).collect())
}

pub async fn get_articles(
    user_id: &str,
    class_id: &str,
    page_size: Option<i32>,
) -> Result<Vec<Article>> {
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
    let resp: GetArticlesResponse =
        request::<GetArticlesResponse>(JyhCode::GetArticles, Box::new(data)).await?;
    
    // Ok(resp.iter().map(Into::into<ArticleQuestion>).collect())
    
    todo!()
}


pub async fn get_article_question(article_id: &str) -> Result<ArticleQuestion> {
    todo!()
}


pub async fn submit_article(article: Article, article_question: ArticleQuestion, submit_date_time: Option<&str>) -> Result<()> {
    todo!()
}