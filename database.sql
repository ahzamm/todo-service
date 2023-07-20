-- Drop tables if they exist
DROP TABLE IF EXISTS todo_item;
DROP TABLE IF EXISTS todo_list;

-- Create todo_list table
CREATE TABLE todo_list (
  id SERIAL PRIMARY KEY,
  title VARCHAR(150) NOT NULL
);

-- Create todo_item table
CREATE TABLE todo_item (
  id SERIAL PRIMARY KEY,
  title VARCHAR(150) NOT NULL,
  checked BOOLEAN NOT NULL DEFAULT FALSE,
  list_id INTEGER NOT NULL,
  FOREIGN KEY (list_id) REFERENCES todo_list(id)
);

-- Insert data into todo_list table
INSERT INTO todo_list (title) VALUES ('List 1'), ('List 2');

-- Insert data into todo_item table
INSERT INTO todo_item (title, list_id) VALUES
  ('item 1', 1),
  ('Item 2', 1),
  ('Item 1', 2);
