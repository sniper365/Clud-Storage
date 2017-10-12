CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    file_name VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('files');

CREATE TABLE file_folder_user (
    id SERIAL PRIMARY KEY,
    file_id int4 references files(id) NOT NULL,
    folder_id int4 references folders(id) NOT NULL,
    user_id int4 references users(id) NOT NULL
);
