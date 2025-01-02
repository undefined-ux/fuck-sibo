use crate::error::SiboError;
use crate::tests::{setup, Configuration};
use crate::{get_article_questions, get_articles, get_classes, login, search_school, submit_article};
use chrono::{Local, Timelike};

#[tokio::test]
async fn test_login() {
    let configuration: Configuration = setup();
    let user_information = login(
        configuration.login_name.as_str(),
        configuration.password.as_str(),
        configuration.school_id.as_str(),
    )
    .await
    .unwrap();
    assert_eq!(user_information.school_name, configuration.school_name);
}

#[tokio::test]
async fn test_login_failed() {
    let configuration: Configuration = setup();
    let user_information = login(
        configuration.login_name.as_str(),
        configuration.password.as_str(),
        "123",
    )
    .await;
    assert!(user_information.is_err(), "The Login Should be Failed.");
}

#[tokio::test]
async fn test_search_school() {
    let configuration: Configuration = setup();
    let school_name = configuration.school_name;
    let except_school_id = configuration.school_id;
    let result = search_school(&school_name).await.unwrap();
    assert!(
        result
            .iter()
            .any(|s| { s.name == school_name && s.id == except_school_id }),
        "School {}(ID: {}) not found",
        school_name,
        except_school_id
    );
}

#[tokio::test]
async fn test_get_class_id() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let res = get_classes(user_id.as_str()).await.unwrap();
    let except_id = configuration.class_id;
    assert!(
        res.iter().any(|c| { c.id == except_id }),
        "Except Class not found"
    );
}

#[tokio::test]
async fn test_get_article_with_page_size() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(10), None)
        .await
        .unwrap();
    assert_eq!(
        res.len(),
        10,
        "Number of articles should be 10, now get {}",
        res.len()
    );
}

#[tokio::test]
async fn test_get_article_with_negative_page_size() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(-1), None).await;
    assert!(res.is_err(), "Page size could not be negative.");
}

#[tokio::test]
async fn test_get_article_without_page_size() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), None, None)
        .await
        .unwrap();
    assert!(!res.is_empty(), "Article list should not be empty.");
}

#[tokio::test]
async fn test_get_article_questions() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(1), Some(true))
        .await
        .unwrap();
    let article = match res.first() {
        Some(article) => article,
        None => panic!("No article found."),
    };
    assert!(article.questions.is_some(), "No questions found.");
}

#[tokio::test]
async fn test_can_submit_article() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let time = Local::now().with_nanosecond(0).unwrap();
    let submit_datetime = time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(1), Some(true))
        .await
        .unwrap();
    let article = match res.first() {
        Some(article) => article.clone(),
        None => panic!("No article found."),
    };
    match submit_article(&user_id, &class_id, &article, Some(&submit_datetime)).await {
        Ok(_) => {}
        Err(e) => match e {
            SiboError::SubmitFailed { message, .. } => {
                assert_eq!(
                    message, "该文章测试已提交，请勿重复提交",
                    "Failed to Submit Article {}",
                    message
                )
            }
            _ => {
                panic!("Failed to submit article. {}", e)
            }
        },
    }
}

#[tokio::test]
async fn test_can_submit_article_without_submit_datetime() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(1), Some(true))
        .await
        .unwrap();
    let article = match res.first() {
        Some(article) => article.clone(),
        None => panic!("No article found."),
    };
    match submit_article(&user_id, &class_id, &article, None).await {
        Ok(_) => {}
        Err(e) => match e {
            SiboError::SubmitFailed { message, .. } => {
                assert_eq!(
                    message, "该文章测试已提交，请勿重复提交",
                    "Failed to Submit Article {}",
                    message
                )
            }
            _ => {
                panic!("Failed to submit article. {}", e)
            }
        },
    }
}

#[tokio::test]
async fn test_can_submit_article_without_precrawl_questions() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let time = Local::now().with_nanosecond(0).unwrap();
    let submit_datetime = time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(1), Some(false))
        .await
        .unwrap();
    let article = match res.first() {
        Some(article) => article.clone(),
        None => panic!("No article found."),
    };
    match submit_article(&user_id, &class_id, &article, Some(&submit_datetime)).await {
        Ok(_) => {}
        Err(e) => match e {
            SiboError::SubmitFailed { message, .. } => {
                assert_eq!(
                    message, "该文章测试已提交，请勿重复提交",
                    "Failed to Submit Article {}",
                    message
                )
            }
            _ => {
                panic!("Failed to submit article. {}", e)
            }
        },
    }
}


#[tokio::test]
async fn test_get_article_questions_with_already_crawled() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(1), Some(false))
        .await
        .unwrap();
    let mut article = match res.first() {
        Some(article) => article.clone(),
        None => panic!("No article found."),
    };
    article.questions = Some(vec![]);
    let new_article = get_article_questions(article).await.unwrap();
    assert!(new_article.questions.unwrap().is_empty(), "should not be recrawl.");
}
