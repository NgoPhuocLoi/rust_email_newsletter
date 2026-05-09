-- Add migration script here

CREATE TABLE subscription_tokens (
    subscription_token TEXT NOT NULL PRIMARY KEY,
    subscription_id INTEGER NOT NULL REFERENCES subscription(id) ON DELETE CASCADE
)
