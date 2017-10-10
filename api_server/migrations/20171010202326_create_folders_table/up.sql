CREATE TABLE folders (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128),
    parent_id SERIAL references folders(id),
    user_id SERIAL references users(id),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('folders');
