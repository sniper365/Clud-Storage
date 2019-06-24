-- This file should undo anything in `up.sql`
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128)
);

INSERT INTO roles (name) VALUES ('admin');
INSERT INTO roles (name) VALUES ('user');
INSERT INTO roles (name) VALUES ('guest');

CREATE TABLE role_user (
    id SERIAL PRIMARY KEY,
    role_id int4 references roles(id) NOT NULL,
    user_id int4 references users(id) NOT NULL
);

ALTER TABLE users DROP COLUMN role;
