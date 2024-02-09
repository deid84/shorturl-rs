-- Add migration script here
CREATE TABLE "user" (
    id VARCHAR(4) PRIMARY KEY,
    long_url VARCHAR NOT NULL
);