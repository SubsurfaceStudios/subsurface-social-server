-- Your SQL goes here

CREATE TABLE IF NOT EXISTS posts (
    id          UUID         PRIMARY KEY,
    title       TEXT,         
    body        TEXT         NOT NULL,
    author      UUID         NOT NULL REFERENCES authentication (id),
    created     TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated     TIMESTAMP    NOT NULL DEFAULT NOW(),
    deleted     TIMESTAMP,
    attachments JSONB        NOT NULL DEFAULT '[]'
)