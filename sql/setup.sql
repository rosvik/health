CREATE TABLE health_checks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    name TEXT,
    -- HTTP status code
    status INTEGER NOT NULL,
    -- The response time in milliseconds
    response_time INTEGER NOT NULL
);
