-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS User (
    id                  UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name                VARCHAR(50) NOT NULL,
    email               VARCHAR(50) NOT NULL,
    password            VARCHAR(255),
    avatar              VARCHAR(255),
    email_verified_at   TIMESTAMP WITH TIME ZONE,
    activated           BOOLEAN NOT NULL DEFAULT (1),
    role                VARCHAR(50) DEFAULT ('user'),
    -- Settings --
    locale              VARCHAR(10) DEFAULT ('fr')
);

CREATE UNIQUE INDEX IF NOT EXISTS user_name_unique_index ON User (name);
CREATE UNIQUE INDEX IF NOT EXISTS user_email_unique_index ON User (email);

CREATE TABLE IF NOT EXISTS Session (
    id          UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    ip          VARCHAR(39) NOT NULL,
    user_id     UUID,
    token       VARCHAR(255),
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT (now()),
    expires_at  TIMESTAMP WITH TIME ZONE,
    -- Foreign key to user --
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES User(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS session_token ON Session (token) INCLUDE (user_id, ip);
CREATE INDEX IF NOT EXISTS session_ip ON Session (ip);