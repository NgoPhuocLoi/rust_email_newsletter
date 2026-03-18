CREATE TABLE IF NOT EXISTS subscription (
    id serial PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);