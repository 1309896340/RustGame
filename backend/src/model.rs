use serde::{Deserialize, Serialize};

// 用作前端请求格式解析
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuth{
    pub username: String,
    pub password: String
}

// 用作后端获取验证密码
#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub id: u32,
    pub username: String,
    pub password: String,
    pub create_time: String,
    pub update_time: String,
}

// 用作向前端响应结果
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo{
    pub username: String,
    pub create_time: String,
    pub update_time: String,
}
