-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255)
);
