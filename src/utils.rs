use crate::error::SiboError;
use crate::model::{BaseRequestBodyBuilder, BaseResponseBody, BaseResponseBodyResult, JyhCode};
use crate::SiboResult;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display};

const BASE_URL: &str = "http://englishservice.siboenglish.com//MobService/index";
const USER_AGENT: &str = "okhttp/3.12.12";

fn build_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip"));
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}

fn build_request_client() -> SiboResult<reqwest::Client> {
    let header = build_header();
    match reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(header)
        .gzip(true)
        .build()
    {
        Ok(client) => Ok(client),
        Err(e) => panic!("Circuital Failure: {:?}", e),
    }
}

pub(crate) async fn request<Ty: DeserializeOwned + Debug>(
    jyh_code: JyhCode,
    data: Box<dyn Display>,
) -> std::result::Result<Ty, SiboError> {
    let client = build_request_client()?;
    let body = BaseRequestBodyBuilder::default()
        .jyh(String::from(jyh_code))
        .parm(data.to_string())
        .build()
        .unwrap();

    let response = match client.get(BASE_URL).form(&body).send().await {
        Ok(response) => response,
        Err(e) => return Err(SiboError::NetworkError(e)),
    };

    let response_text = match response.text_with_charset("utf-8").await {
        Ok(text) => text,
        Err(e) => return Err(SiboError::NetworkError(e)),
    };

    let response_data: BaseResponseBody = match serde_json::from_str(&response_text) {
        Ok(result) => result,
        Err(e) => return Err(SiboError::JsonParseError(e)),
    };

    let result: BaseResponseBodyResult<Ty> = match serde_json::from_str(&response_data.result) {
        Ok(res) => res,
        Err(err) => return Err(SiboError::JsonParseError(err)),
    };

    if result.code == "1" {
        match result.data {
            Some(data) => Ok(data),
            None => Err(SiboError::RequestFailed {
                jyh: jyh_code,
                error_code: result.code,
                error_message: String::from("Empty Data field"),
            }),
        }
    } else {
        Err(SiboError::RequestFailed {
            jyh: jyh_code,
            error_code: result.code,
            error_message: result.msg,
        })
    }
}
