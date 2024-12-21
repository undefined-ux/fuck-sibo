#![allow(unused)]
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub(crate) struct BaseResponseBodyResult<T>
where
    Self: Sized,
{
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Msg")]
    pub msg: String,
    #[serde(rename = "Data")]
    pub data: Option<T>,
    #[serde(rename = "Num")]
    pub num: i32,
}

#[derive(Serialize, Debug, Deserialize)]
pub(crate) struct BaseResponseBody
where
    Self: Sized,
{
    pub result: String,
    pub sign: String,
    pub ts: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginResponse {
    #[serde(rename = "Login")]
    pub login: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
    #[serde(rename = "UserPic")]
    pub user_pic: String,
    #[serde(rename = "TS")]
    pub ts: i32,
    #[serde(rename = "DeptID")]
    pub dept_id: String,
    #[serde(rename = "IsAdministrator")]
    pub is_administrator: i32,
    #[serde(rename = "SchoolName")]
    pub school_name: String,
    #[serde(rename = "SchoolBH")]
    pub school_bh: String,
    #[serde(rename = "ClassBH")]
    pub class_bh: Option<String>,
    #[serde(rename = "StudentBH")]
    pub student_bh: String,
    #[serde(rename = "TermID")]
    pub term_id: String,
    #[serde(rename = "TermBH")]
    pub term_bh: String,
    #[serde(rename = "TermName")]
    pub term_name: String,
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "RoleCode")]
    pub role_code: Option<String>,
    #[serde(rename = "Type")]
    pub _type: i32,
    #[serde(rename = "LimitData")]
    pub limit_data: String,
    #[serde(rename = "IsLimit")]
    pub is_limit: i32,
    #[serde(rename = "PowerLimit")]
    pub power_limit: i32,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "UpdateFields")]
    pub update_fields: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetSchoolIDResult {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "SchoolName")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetStudentClassIDResult {
    #[serde(rename = "ClassID")]
    pub class_id: String,
    #[serde(rename = "ClassBH")]
    pub class_bh: String,
    #[serde(rename = "termName")]
    pub term_name: Option<String>,
    #[serde(rename = "ClassName")]
    pub class_name: String,
    #[serde(rename = "TeacherName")]
    pub teacher_name: String,
    #[serde(rename = "CourseSign")]
    pub course_sign: i32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetArticlesResult {
    #[serde(rename = "EssayID")]
    pub essay_id: String,
    #[serde(rename = "EssayType")]
    pub essay_type: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "Picurl")]
    pub picture_url: String,
    #[serde(rename = "Grade")]
    pub grade: i32,
    #[serde(rename = "RGLLevel")]
    pub rgl_level: String,
    pub sign: i32,
    #[serde(rename = "readParagraph")]
    pub read_paragraph: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetArticlesQuestionsResponse {
    #[serde(rename = "TestID")]
    pub test_id: String,
    #[serde(rename = "TestItemNumber")]
    pub test_item_number: i32,
    #[serde(rename = "TestItemType")]
    pub test_item_type: i32,
    #[serde(rename = "TestItemTitle")]
    pub test_item_title: String,
    #[serde(rename = "Options")]
    pub options: String,
    #[serde(rename = "Answer")]
    pub answer: String,
    #[serde(rename = "ChoseA")]
    pub chose_a: String,
    #[serde(rename = "ChoseB")]
    pub chose_b: String,
    #[serde(rename = "ChoseC")]
    pub chose_c: String,
    #[serde(rename = "ChoseD")]
    pub chose_d: String,
    #[serde(rename = "Analysis")]
    pub analysis: String,
    #[serde(rename = "MyAnswer")]
    pub my_answer: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ReadArticleResponseModel {
    #[serde(rename = "Introductory")]
    pub introductory: String,
    #[serde(rename = "Music")]
    pub music: String,
    #[serde(rename = "SectionCount")]
    pub section_count: i32,
    #[serde(rename = "ContentLength")]
    pub content_length: i32,
    #[serde(rename = "Browsenum")]
    pub browse_num: i32,
    #[serde(rename = "ExerciseState")]
    pub exercise_state: i32,
    #[serde(rename = "EssayID")]
    pub essay_id: String,
    #[serde(rename = "EssayType")]
    pub essay_type: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "Picurl")]
    pub picurl: String,
    #[serde(rename = "Grade")]
    pub grade: i32,
    #[serde(rename = "RGLLevel")]
    pub rgl_level: String,
    pub sign: i32,
    #[serde(rename = "readParagraph")]
    pub read_paragraph: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ReadArticleResponse {
    pub model: ReadArticleResponseModel,
    #[serde(rename = "isShow")]
    pub is_show: String,
    #[serde(rename = "showContent")]
    pub show_content: String,
}

pub(crate) type GetArticlesResponse = Vec<GetArticlesResult>;
pub(crate) type GetStudentClassIDResponse = Vec<GetStudentClassIDResult>;
pub(crate) type GetSchoolIDResponse = Vec<GetSchoolIDResult>;


impl_from_string!(BaseResponseBody);
impl_from_string!(LoginResponse);
impl_from_string!(GetSchoolIDResult);
impl_from_string!(GetStudentClassIDResult);
impl_from_string!(GetArticlesResult);
impl_from_string!(GetArticlesQuestionsResponse);
impl_from_string!(ReadArticleResponseModel);
impl_from_string!(ReadArticleResponse);
