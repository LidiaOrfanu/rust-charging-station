-- Add migration script here
CREATE TABLE penguins (
    id  SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    species varchar(255) NOT NULL,
    age  INTEGER
);