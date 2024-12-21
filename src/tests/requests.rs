use crate::{login, search_school};
use crate::tests::{setup, Configuration};


#[tokio::test]
async fn test_login() {
    let configuration: Configuration = setup();
    let user_information = login(
        configuration.login_name.as_str(), 
        configuration.password.as_str(), 
        configuration.school_id.as_str()
    ).await.unwrap();
    println!("User information: {:?}", user_information);
    assert_eq!(user_information.school_name, configuration.school_name);
}

#[tokio::test]
async fn test_search_school() {
    let configuration: Configuration = setup();
    let school_name = configuration.school_name;
    let except_school_id = configuration.school_id;
    let result = search_school(&school_name).await.unwrap();
    assert!(result.iter().any(|s| {
        s.name == school_name && s.id == except_school_id
    }), "School {}(ID: {}) not found", school_name, except_school_id);
}

#[tokio::test]
async fn test_get_class_id() {
}