-- Create a table for the health check
CREATE TABLE endpoints (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

	url TEXT NOT NULL,
	-- The interval in milliseconds
	interval INTEGER NOT NULL
);

CREATE TABLE observation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    endpoint_id INTEGER NOT NULL,
    status INTEGER NOT NULL,
		-- The response time in milliseconds
		response_time INTEGER NOT NULL,

    FOREIGN KEY (endpoint_id) REFERENCES endpoints (id)
);
