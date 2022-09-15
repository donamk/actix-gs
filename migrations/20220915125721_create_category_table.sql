-- Add migration script here
CREATE TABLE category(
     id SERIAL PRIMARY KEY,
     name VARCHAR(30) NOT NULL,
     parent_id INT NOT NULL,
     created_at TIMESTAMP WITH TIME ZONE NOT NULL,
     updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
