use axum::{Router, extract::Json, http::StatusCode, routing::{get, post}};

// mod model;
use crate::model::User;

async fn hello() -> String {
    // 相应根get请求
    return String::from("账号认证服务器");
}

async fn register(Json(user): Json<User>) -> (StatusCode, Json<User>){
    // 接收post请求
    println!("接收到请求的user为\n{:#?}", user);
    (StatusCode::CREATED, Json(user))
}

async fn login(){
    // 接收post请求
}


pub fn construct_router() -> Router{
    Router::new()
    .route("/", get(hello))
    .route("/register", post(register))
    .route("/login", post(login))
}