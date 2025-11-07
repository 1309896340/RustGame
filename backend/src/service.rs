// use axum::Error;
use base64::{Engine, engine::general_purpose::STANDARD};
use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::usize;

use crate::error::ServiceError;
use crate::model::{User, UserAuth, UserInfo};

// 数据库操作服务
pub fn select_user(username: &str) -> Result<User> {
    // 获取User完整信息
    let db_path_buf = PathBuf::new()
        .join(std::env::current_dir().expect("当前工作目录"))
        .join("backend")
        .join("sqlite")
        .join("data.db");
    let db_path = db_path_buf.as_path();
    let conn = Connection::open(db_path)?;
    let res = conn.query_row("SELECT id, username, password, create_time, update_time FROM user WHERE username = ? AND del_flag = 0", (username,),|row|{
        Ok(User{
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            create_time: row.get(3)?,
            update_time: row.get(4)?
        })
    });
    let _ = conn.close();
    return res;
}

pub fn insert_user(user: &UserAuth) -> Result<usize> {
    // 创建新用户
    let db_path_buf = PathBuf::new()
        .join(std::env::current_dir().expect("当前工作目录"))
        .join("backend")
        .join("sqlite")
        .join("data.db");
    let db_path = db_path_buf.as_path();
    let conn = Connection::open(db_path)?;
    let res = conn.execute(
        "INSERT INTO user (username, password) VALUES (?, ?)",
        (&user.username, &user.password),
    );
    let _ = conn.close();
    return res;
}

// 业务服务
pub fn get_user_info(username: &str) -> Result<UserInfo, ServiceError> {
    match select_user(username) {
        Ok(user) => Ok(UserInfo {
            username: user.username,
            create_time: user.create_time,
            update_time: user.update_time,
        }),
        Err(_) => Err(ServiceError::NotFound(format!(
            "找不到用户\"{}\"",
            username
        ))),
    }
}

fn string_to_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash_result: [u8; 32] = hasher.finalize().into();
    let base_encoded_hash = STANDARD.encode(&hash_result);
    return base_encoded_hash;
}

pub fn register(user: &UserAuth) -> Result<(), ServiceError> {
    match insert_user(user) {
        Ok(_usize) => Ok(()),
        Err(err) => Err(ServiceError::RegisterFailed(format!("注册失败: {}", err))),
    }
}


pub fn login(user: &UserAuth) -> Result<(), ServiceError> {
    match select_user(&user.username) {
        Ok(t_user) => match string_to_sha256(&user.password) == t_user.password {
            true => Ok(()),
            false => Err(ServiceError::LoginFailed(format!("登录失败: 密码错误"))),
        },
        Err(_err) => Err(ServiceError::LoginFailed(format!("登录失败: 找不到用户"))),
    }
}
