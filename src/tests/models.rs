use crate::model::*;

#[test]
fn test_can_generate_correct_login_request_body_param() {
    let params = match LoginRequestBodyParamBuilder::default()
        .login_name("test".to_string())
        .password("test".to_string())
        .school_id("test".to_string())
        .build()
    {
        Ok(params) => params,
        Err(e) => panic!("Error: {}", e),
    };
    let json_str: String = format!("{}", params);
    assert_eq!(
        json_str,
        r#"{"schoolID":"test","loginName":"test","password":"test","ts":2,"appVersion":"4.5.0"}"#
    );
}

#[test]
fn test_can_generate_correct_get_school_information_param() {
    let params = match GetSchoolInformationParamBuilder::default()
        .keyword("test".to_string())
        .build()
    {
        Ok(params) => params,
        Err(e) => panic!("Error: {}", e),
    };
    let json_str: String = format!("{}", params);
    assert_eq!(
        json_str,
        r#"{"keyWord":"test","pageStart":0,"pageSize":2147483647}"#
    );
}

#[test]
fn test_can_generate_correct_get_class_information_param() {
    let params = match GetClassInformationParamBuilder::default()
        .user_id("test".to_string())
        .build()
    {
        Ok(params) => params,
        Err(e) => panic!("Error: {}", e),
    };
    let json_str: String = format!("{}", params);
    assert_eq!(json_str, r#"{"ts":2,"userID":"test"}"#);
}

#[test]
fn test_can_generate_correct_get_articles_param() {
    let params = match GetArticlesParamBuilder::default()
        .user_id("test".to_string())
        .class_id("test".to_string())
        .page_size(10)
        .build()
    {
        Ok(params) => params,
        Err(e) => panic!("Error: {}", e),
    };
    let json_str: String = format!("{}", params);
    assert_eq!(
        json_str,
        r#"{"ts":2,"userID":"test","classID":"test","pageSize":10,"pageStart":0,"orderType":1,"grade":0,"eassyType":"","keyWord":""}"#
    );
}

#[test]
fn test_can_generate_correct_get_articles_questions_param() {
    let params = match GetArticlesQuestionsParamBuilder::default()
        .essay_id("test".to_string())
        .build()
    {
        Ok(params) => params,
        Err(e) => panic!("Error: {}", e),
    };
    let json_str: String = format!("{}", params);
    assert_eq!(json_str, r#"{"essayID":"test"}"#);
}

#[test]
fn test_can_generate_correct_read_articles_param() {
    let params = match ReadArticlesParamBuilder::default()
        .user_id("test".to_string())
        .class_id("test".to_string())
        .essay_id("test".to_string())
        .build()
    {
        Ok(params) => params,
        Err(e) => panic!("Error: {}", e),
    };
    let json_str: String = format!("{}", params);
    assert_eq!(
        json_str,
        r#"{"essayID":"test","userID":"test","classID":"test"}"#
    );
}

#[test]
fn test_can_generate_correct_submit_articles_tests_param() {
    let params = SubmitArticlesParamBuilder::default()
        .essay_id("test".to_string())
        .user_id("test".to_string())
        .class_id("test".to_string())
        .create_time("test".to_string())
        .answer("test".to_string())
        .build()
        .unwrap();
    let json_str: String = params.to_string();
    assert_eq!(
        json_str,
        r#"{"essayID":"test","userID":"test","classID":"test","createTime":"test","itemResult":"test"}"#
    );
}

#[test]
fn test_login_response_into_user_information() {
    let login_response: LoginResponse = LoginResponse {
        login: "test".to_string(),
        user_name: "test".to_string(),
        user_pic: "test".to_string(),
        ts: 2,
        dept_id: "test".to_string(),
        is_administrator: 0,
        school_name: "test university".to_string(),
        school_bh: "312".to_string(),
        class_bh: None,
        student_bh: "test".to_string(),
        term_id: "test".to_string(),
        term_bh: "test".to_string(),
        term_name: "test".to_string(),
        phone: "test".to_string(),
        email: None,
        role_code: None,
        _type: 0,
        limit_data: "test".to_string(),
        is_limit: 0,
        power_limit: 0,
        id: "test".to_string(),
        update_fields: vec![],
    };
    let result = UserInformation::from(login_response);
    let except_result = UserInformation {
        name: "test".to_string(),
        school_name: "test university".to_string(),
        id: "test".to_string(),
    };

    assert_eq!(result.name, except_result.name);
    assert_eq!(result.school_name, except_result.school_name);
    assert_eq!(result.id, except_result.id);
}

#[test]
fn test_get_school_id_result_into_school_information() {
    let school_id_result = GetSchoolIDResult {
        id: "test".to_string(),
        name: "test".to_string(),
    };

    let school_information = Into::<SchoolInformation>::into(school_id_result);
    let except_result = SchoolInformation {
        id: "test".to_string(),
        name: "test".to_string(),
    };
    assert_eq!(school_information.id, except_result.id);
    assert_eq!(school_information.name, except_result.name);
}

#[test]
fn test_string_parse_into_base_response_body() {
    let str: &str = r#"{"result": "test", "sign": "2", "ts": "2"}"#;
    let base_response_body: BaseResponseBody = BaseResponseBody::from(str.to_string());
    let except_result = BaseResponseBody {
        result: "test".to_string(),
        sign: "2".to_string(),
        ts: "2".to_string(),
    };

    assert_eq!(except_result.result, base_response_body.result);
    assert_eq!(except_result.sign, base_response_body.sign);
    assert_eq!(except_result.ts, base_response_body.ts);
}

#[test]
fn test_jyh_code_to_string() {
    assert_eq!(String::from(JyhCode::Login), "4002_01".to_string());
    assert_eq!(
        String::from(JyhCode::SearchSchoolInformation),
        "4001".to_string()
    );
    assert_eq!(
        String::from(JyhCode::GetClassInformation),
        "1001".to_string()
    );
    assert_eq!(String::from(JyhCode::GetArticles), "2002".to_string());
    assert_eq!(
        String::from(JyhCode::GetArticleQuestions),
        "2009".to_string()
    );
    assert_eq!(String::from(JyhCode::ReadArticle), "2003".to_string());
    assert_eq!(String::from(JyhCode::SubmitArticle), "2010".to_string());
}

#[test]
fn test_get_student_class_id_result_into_class_information() {
    let get_student_class_id_result = GetStudentClassIDResult {
        class_id: "test".to_string(),
        class_bh: "".to_string(),
        term_name: None,
        class_name: "test".to_string(),
        teacher_name: "test".to_string(),
        course_sign: 0,
    };
    let result = ClassInformation::from(get_student_class_id_result);
    let except_result = ClassInformation {
        id: "test".to_string(),
        term_name: None,
        name: "test".to_string(),
        teacher_name: "test".to_string(),
    };

    assert_eq!(result.id, except_result.id);
    assert_eq!(result.teacher_name, except_result.teacher_name);
    assert_eq!(result.name, except_result.name);
    assert_eq!(result.term_name, except_result.term_name);
}

#[test]
fn test_get_articles_result_into_article_information() {
    let article = GetArticlesResult {
        essay_id: "test".to_string(),
        essay_type: "test".to_string(),
        title: "test".to_string(),
        create_time: "".to_string(),
        picture_url: "".to_string(),
        grade: 0,
        rgl_level: "".to_string(),
        sign: 0,
        read_paragraph: "".to_string(),
    };

    let result = Article::new(article, None);
    let except_result = Article {
        title: "test".to_string(),
        difficulty: 0,
        id: "test".to_string(),
        article_type: "test".to_string(),
        questions: None,
        answer: None,
    };

    assert_eq!(result.title, except_result.title);
    assert_eq!(result.difficulty, except_result.difficulty);
    assert_eq!(result.id, except_result.id);
    assert_eq!(result.article_type, except_result.article_type);
}

#[test]
fn test_get_article_questions_result_into_article_question() {
    let question = GetArticlesQuestionsResult {
        test_id: "test".to_string(),
        test_item_number: 0,
        test_item_type: 0,
        test_item_title: "test".to_string(),
        options: "test".to_string(),
        answer: "A".to_string(),
        chose_a: "A".to_string(),
        chose_b: "B".to_string(),
        chose_c: "C".to_string(),
        chose_d: "D".to_string(),
        analysis: "test".to_string(),
        my_answer: None,
    };
    let result = ArticleQuestion::from(question);
    let except_result = ArticleQuestion {
        title: "test".to_string(),
        choices: vec!["A".into(), "B".into(), "C".into()],
        answer: "A".to_string(),
        index: 0,
        id: "test".to_string(),
        analysis: "test".to_string(),
        question_type: 0,
    };

    assert_eq!(result.answer, except_result.answer);
    assert_eq!(result.question_type, except_result.question_type);
    assert_eq!(result.title, except_result.title);
    assert_eq!(result.id, except_result.id);
    assert_eq!(result.analysis, except_result.analysis);
    assert_eq!(result.question_type, except_result.question_type);
}
