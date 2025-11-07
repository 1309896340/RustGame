use axum::{Router, extract::Json, http::StatusCode, routing::{get, post}};

use crate::model::UserAuth;
use crate::service::{get_user_info, register, login};

async fn hello()->String{
    // 相应根get请求
    String::from("账号认证服务器")
}

async fn user_register(Json(user): Json<UserAuth>) -> (StatusCode, String){
    println!("接收到注册请求为\n{:#?}", user);
    let res = match register(&user){
        Ok(()) => format!("注册成功"),
        Err(error) => format!("发生错误: {:#?}", error)
    };

    (StatusCode::CREATED, res)
}

async fn user_login(Json(user): Json<UserAuth>) -> (StatusCode, String) {
    println!("接收到登录请求为\n{:#?}", user);
    match login(&user){
        Ok(()) => (StatusCode::OK, format!("{} 登录成功!", user.username)),
        Err(error) => (StatusCode::EXPECTATION_FAILED, format!("{} 登录失败: {:#?}", user.username, error))
    }
}


pub fn construct_router() -> Router{
    Router::new()
    .route("/", get(hello))
    .route("/register", post(user_register))
    .route("/login", post(user_login))
}