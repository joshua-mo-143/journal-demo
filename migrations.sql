CREATE TABLE IF NOT EXISTS entries (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	body VARCHAR NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
