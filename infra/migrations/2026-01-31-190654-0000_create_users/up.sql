-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
                         id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
                         username VARCHAR(64) NOT NULL,
                         personal_email VARCHAR(70) NOT NULL UNIQUE,
                         salt bytea NOT NULL,
                         password_mac bytea NOT NULL,
                         password_sha1 bytea NOT NULL
);
