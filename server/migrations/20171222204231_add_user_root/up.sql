ALTER TABLE users ADD COLUMN root INT4 references folders(id);
