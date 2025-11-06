use rusqlite::{Connection, Result};
use std::path::PathBuf;

use crate::model::{User,UserAuth};

pub fn get_user(username: &str) -> Result<User> {
    let db_path_buf = PathBuf::new().join(std::env::current_dir().expect("当前工作目录")).join("backend").join("sqlite").join("data.db");
    let db_path = db_path_buf.as_path();
    let conn = Connection::open(db_path)?;
    let res = conn.query_row("SELECT id, username, password, create_time, update_time FROM user WHERE username = ? AND del_flag = 1", (username,),|row|{
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

pub fn create_user(user: UserAuth) -> Result<(usize)>{
    let db_path_buf = PathBuf::new().join(std::env::current_dir().expect("当前工作目录")).join("backend").join("sqlite").join("data.db");
    let db_path = db_path_buf.as_path();
    let conn = Connection::open(db_path)?;
    let res= conn.execute("INSERT INTO user (username, password) VALUES (?, ?)", (user.username, user.password));
    let _ = conn.close();
    return res;
}
