use derive_builder::Builder;
use serde::{Deserialize, Serialize};
const DEFAULT_TS_VALUE: i32 = 2;
const DEFAULT_APP_VERSION: &str = "4.5.0";

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct BaseRequestBody
where
    Self: Sized,
{
    pub jyh: String,
    pub parm: String,
    #[builder(setter(skip = true), default = "String::new()")]
    pub sign: String,
    #[builder(setter(skip = true), default = "String::new()")]
    pub ts: String,
}

#[derive(Serialize, Deserialize, Debug, Builder)]
pub(crate) struct LoginRequestBodyParam {
    #[serde(rename = "schoolID")]
    pub school_id: String,
    #[serde(rename = "loginName")]
    pub login_name: String,
    pub password: String,
    #[builder(setter(skip = true), default = "DEFAULT_TS_VALUE")]
    pub ts: i32,
    #[serde(rename = "appVersion")]
    #[builder(setter(skip = true), default = "DEFAULT_APP_VERSION.to_string()")]
    pub app_version: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct GetSchoolInformationParam {
    #[serde(rename = "keyWord")]
    keyword: String,
    #[serde(rename = "pageStart")]
    #[builder(setter(skip = true), default = "0")]
    page_start: i32,
    #[serde(rename = "pageSize")]
    #[builder(setter(skip = true), default = "i32::MAX")]
    page_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct GetClassInformationParam {
    #[builder(setter(skip = true), default = "DEFAULT_TS_VALUE")]
    ts: i32,
    #[serde(rename = "userID")]
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct GetArticlesParam {
    #[builder(setter(skip = true), default = "DEFAULT_TS_VALUE")]
    ts: i32,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "classID")]
    class_id: String,
    #[serde(rename = "pageSize")]
    #[builder(default = "10")]
    page_size: i32,
    #[serde(rename = "pageStart")]
    #[builder(setter(skip = true), default = "0")]
    page_start: i32,
    #[serde(rename = "orderType")]
    #[builder(setter(skip = true), default = "1")]
    order_type: i32,
    #[builder(default = "0")]
    grade: i32,
    #[serde(rename = "eassyType")] // API params misspelled.
    #[builder(setter(skip = true), default = "String::new()")]
    essay_type: String,
    #[serde(rename = "keyWord")]
    #[builder(setter(skip = true), default = "String::new()")]
    keyword: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct GetArticlesQuestionsParam {
    #[serde(rename = "essayID")]
    pub essay_id: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct ReadArticlesParam {
    #[serde(rename = "essayID")]
    pub essay_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "classID")]
    pub class_id: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub(crate) struct SubmitArticlesParam {
    #[serde(rename = "essayID")]
    pub essay_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "classID")]
    pub class_id: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "itemResult")]
    pub answer: String,
}

impl_display!(LoginRequestBodyParam);
impl_display!(GetSchoolInformationParam);
impl_display!(GetClassInformationParam);
impl_display!(GetArticlesParam);
impl_display!(GetArticlesQuestionsParam);
impl_display!(ReadArticlesParam);
impl_display!(SubmitArticlesParam);
