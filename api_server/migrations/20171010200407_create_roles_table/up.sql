-- up.sql
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128)
);

INSERT INTO roles (name) VALUES ('admin');
INSERT INTO roles (name) VALUES ('user');
INSERT INTO roles (name) VALUES ('guest');

CREATE TABLE role_user (
    id SERIAL PRIMARY KEY,
    role_id SERIAL references roles(id),
    user_id SERIAL references users(id)
);
