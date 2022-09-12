CREATE TABLE IF NOT EXISTS torrust_user_bans (
    ban_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    reason TEXT NOT NULL,
    date_expiry TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES torrust_users(user_id) ON DELETE CASCADE
)
