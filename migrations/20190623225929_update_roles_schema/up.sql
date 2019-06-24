-- Your SQL goes here

ALTER TABLE users ADD COLUMN role VARCHAR(6) DEFAULT 'user' NOT NULL;

UPDATE users SET role = 'user';

UPDATE users SET role = 'admin' WHERE id IN (
    SELECT user_id FROM role_user WHERE role_id IN (
        SELECT id FROM roles WHERE name = 'admin'
    )
);

DROP TABLE role_user;
DROP TABLE roles;
