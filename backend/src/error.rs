use rusqlite;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError{
  // 业务错误: 用户不存在
  #[error("找不到用户id: {0}")]
  NotFound(String),

  // 业务错误: 密码错误，用户不存在等
  #[error("登录失败: {0}")]
  LoginFailed(String),

  // 业务错误: 同名用户存在，用户名、密码不合法等
  #[error("注册失败: {0}")]
  RegisterFailed(String),

  // 包装底层的错误，通常会实现 From<T> trait
  #[error("数据库错误: {0}")]
  DatabaseError(#[from] rusqlite::Error),

  // 底层错误: 其他IO错误
  #[error("IO错误: {0}")]
  IoError(#[from] std::io::Error)
}