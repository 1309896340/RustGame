use axum::{Router, extract::Json, http::StatusCode, routing::{get, post}};

use crate::model::{User, UserAuth, UserInfo};
use crate::service::{get_user, create_user};

async fn hello()->String{
    // 相应根get请求
    String::from("账号认证服务器")
}

async fn register(Json(user): Json<UserAuth>) -> (StatusCode, String){
    // 接收post请求
    println!("接收到注册请求为\n{:#?}", user);
    let res = match create_user(user){
        Ok(update_num) => format!("注册成功: {}", update_num),
        Err(error) => format!("发生错误: {:#?}", error)
    };

    (StatusCode::CREATED, res)
}

async fn login(Json(user): Json<UserAuth>) -> (StatusCode, Result<Json<UserInfo>, Box<dyn std::error::Error>>) {
    // 接收post请求
    println!("接收到登录请求为\n{:#?}", user);
    let t_user = match get_user(&user.username){
        Ok(user) => user,
        Err(error) => { return (StatusCode::NOT_FOUND, Err(Box::new(error))); }
    };
    if user.password == t_user.password {
        (StatusCode::OK, Ok(Json(UserInfo{
            username: t_user.username,
            create_time: t_user.create_time,
            update_time: t_user.update_time
        })))
    }else{
        (StatusCode::UNAUTHORIZED, Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "密码错误"))))
    }
}


pub fn construct_router() -> Router{
    Router::new()
    .route("/", get(hello))
    .route("/register", post(register))
    .route("/login", post(login))
}