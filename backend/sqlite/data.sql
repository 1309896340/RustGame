CREATE TABLE user(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
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
