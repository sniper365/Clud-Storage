CREATE TABLE folders (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    parent_id int4 references folders(id) NULL,
    user_id int4 references users(id) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('folders');
