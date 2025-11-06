# Rust高性能游戏服务器

## 技术栈

- 数据库: Sqlite3
- 后端: Axum
- 前端: Vue3
- 客户端: Bevy

## 学习路线

1. 入门Web框架Auxm
2. 验证Sqlite3的CURD操作
3. 入门游戏客户端Bevy框架

### 认证(注册、登录)

1. Auxm设计后端
2. Vue3设计网页前端
3. Sqlite3保存用户数据

#### 一、项目结构

- backend/
  - sqlite/
    - data.db
- frontend/
- Cargo.toml
- package.json

其中处于顶层的Cargo.toml和package.json如下编写:

Cargo.toml

```toml
[workspace]
members = [
    "backend"
]
resolver = "2"
```

package.json

```json
{
    "workspaces":[
        "frontend"
    ],
    "scripts": {
        "start:frontend": "npm start --workspace frontend"
    }
}
```

后续在根目录使用`npm`时可以指定`--prefix frontend`来将命令在子目录中执行

在执行Axum后端时使用

```shell
cargo build --workspace
cargo run -p backend
```

frontend使用tailwindcss进行页面设计

```shell
npm i tailwindcss @tailwindcss/vite --prefix frontend
```



#### 二、创建数据库表结构

```sqlite
CREATE TABLE user(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    create_time TEXT NOT NULL DEFAULT (datetime('now')),
    update_time TEXT NOT NULL DEFAULT (datetime('now')),
    del_flag INTEGER NOT NULL DEFAULT 0
);
CREATE TRIGGER update_user_update_time
AFTER UPDATE ON user
FOR EACH ROW
BEGIN
	UPDATE user SET update_time = datetime('now') WHERE id = OLD.id;
END;
```

在rust中使用`rusqlite`进行数据库访问
```rust
use rusqlite::{params, Connection, Result};
```

使用`Connection::open()`打开指定的.db文件






