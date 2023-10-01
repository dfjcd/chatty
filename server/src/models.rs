use serde::{Deserialize, Serialize};

pub struct ChatUser {
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub action: String,
    pub data: RequestData,
}

#[derive(Serialize, Deserialize)]
pub enum RequestData {
    #[serde(rename = "data")]
    Login(String),
    #[serde(rename = "data")]
    Message(String),
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub action: String,
    pub data: ResponseData,
}

#[derive(Serialize, Deserialize)]
pub enum ResponseData {
    #[serde(rename = "data")]
    Error(String),
    #[serde(rename = "data")]
    Message(String),
}