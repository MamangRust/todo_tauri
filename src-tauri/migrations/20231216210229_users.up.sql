-- Add up migration script here
CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    email TEXT,
    password TEXT,
    created_at TIMESTAMP DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
    updated_at TIMESTAMP DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
);
