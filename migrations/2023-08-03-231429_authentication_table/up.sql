-- Your SQL goes here
CREATE TABLE IF NOT EXISTS authentication (
    id       UUID         PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    salt     VARCHAR(255) NOT NULL,
    hash     BYTEA        NOT NULL,
    created  TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated  TIMESTAMP    NOT NULL DEFAULT NOW()
);