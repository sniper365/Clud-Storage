CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    file_name VARCHAR(128) NOT NULL,
    folder_id INT4 references folders(id) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('files');
