CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128),
    file_name VARCHAR(128),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('files');

CREATE TABLE file_folder_user (
    file_id SERIAL references files(id),
    folder_id SERIAL references folders(id),
    user_id SERIAL references users(id)
);
