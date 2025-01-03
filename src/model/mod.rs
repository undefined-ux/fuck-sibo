#[macro_use]
pub(crate) mod macros;
pub(crate) mod requests;
pub(crate) mod response;

pub(crate) use requests::*;
pub(crate) use response::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum JyhCode {
    Login,
    SearchSchoolInformation,
    GetClassInformation,
    GetArticles,
    GetArticleQuestions,
    ReadArticle,
    SubmitArticle,
}

impl From<JyhCode> for String {
    fn from(value: JyhCode) -> Self {
        match value {
            JyhCode::Login => "4002_01",
            JyhCode::SearchSchoolInformation => "4001",
            JyhCode::GetClassInformation => "1001",
            JyhCode::GetArticles => "2002",
            JyhCode::GetArticleQuestions => "2009",
            JyhCode::ReadArticle => "2003",
            JyhCode::SubmitArticle => "2010",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct UserInformation {
    pub name: String,
    pub school_name: String,
    pub id: String,
}

impl From<LoginResponse> for UserInformation {
    fn from(value: LoginResponse) -> Self {
        UserInformation {
            name: value.user_name,
            school_name: value.school_name,
            id: value.id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchoolInformation {
    pub name: String,
    pub id: String,
}

impl From<GetSchoolIDResult> for SchoolInformation {
    fn from(value: GetSchoolIDResult) -> Self {
        SchoolInformation {
            name: value.name,
            id: value.id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassInformation {
    pub id: String,
    pub term_name: Option<String>,
    pub name: String,
    pub teacher_name: String,
}

impl From<GetStudentClassIDResult> for ClassInformation {
    fn from(value: GetStudentClassIDResult) -> Self {
        ClassInformation {
            id: value.class_id,
            term_name: value.term_name,
            name: value.class_name,
            teacher_name: value.teacher_name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArticleQuestion {
    pub title: String,
    pub choices: Vec<String>,
    pub answer: String,
    pub index: i32,
    pub id: String,
    pub analysis: String,
    pub question_type: i32,
}

impl From<GetArticlesQuestionsResult> for ArticleQuestion {
    fn from(value: GetArticlesQuestionsResult) -> Self {
        ArticleQuestion {
            title: value.test_item_title,
            choices: vec![value.chose_a, value.chose_b, value.chose_c, value.chose_d],
            answer: value.answer,
            index: value.test_item_number,
            id: value.test_id,
            analysis: value.analysis,
            question_type: value.test_item_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub difficulty: i32,
    pub id: String,
    pub article_type: String,
    pub questions: Option<Vec<ArticleQuestion>>,
    pub answer: Option<String>,
}

impl Article {
    pub(super) fn build_answer_string(questions: &[ArticleQuestion]) -> String {
        let mut answer = questions.iter().fold(String::new(), |str, q| {
            format!("{}{}-{};", str, q.index, q.answer)
        });
        if !answer.is_empty() {
            answer.pop();
        }
        answer
    }
    pub(crate) fn new(
        metadata: GetArticlesResult,
        questions: Option<Vec<ArticleQuestion>>,
    ) -> Self {
        Self {
            title: metadata.title,
            difficulty: metadata.grade,
            id: metadata.essay_id,
            article_type: metadata.essay_type,
            questions: questions.clone(),
            answer: questions.map(|questions| Article::build_answer_string(&questions)),
        }
    }
    pub(crate) fn fill_questions(&mut self, questions: Vec<ArticleQuestion>) {
        self.questions = Some(questions);
        self.answer = Some(Article::build_answer_string(
            self.questions.as_ref().unwrap(),
        ));
    }
}
// impl From<GetArticlesResult> for Article {
//     fn from(value: GetArticlesResult) -> Self {
//         Article {
//             title: value.title,
//             difficulty: value.grade,
//             id: value.essay_id,
//             article_type: value.essay_type
//         }
//     }
// }
//
