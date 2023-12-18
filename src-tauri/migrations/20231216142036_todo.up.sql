-- Add up migration script here
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
    updated_at TIMESTAMP DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
);
