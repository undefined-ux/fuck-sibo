use crate::tests::{setup, Configuration};
use crate::{get_articles, get_classes, login, search_school};

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
    println!("User information: {:?}", user_information);
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
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(10))
        .await
        .unwrap();
    println!("{:#?}", res);
    assert_eq!(res.len(), 10, "Number of articles should be 10, now get {}", res.len());
}


#[tokio::test]
async fn test_get_article_with_negative_page_size() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), Some(-1))
        .await;
    assert!(res.is_err(), "Page size could not be negative.");
}

#[tokio::test]
async fn test_get_article_without_page_size() {
    let configuration: Configuration = setup();
    let user_id = configuration.user_id;
    let class_id = configuration.class_id;
    let res = get_articles(user_id.as_str(), class_id.as_str(), None)
        .await.unwrap();
    assert!(!res.is_empty(), "Article list should not be empty.");
}