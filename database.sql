DROP TABLE IF EXISTS todo_list;
DROP TABLE IF EXISTS todo_item;

CREATE TABLE todo_list (
    id serial PRIMARY KEY,
    title varchar(150) NOT NULL
);

CREATE TABLE todo_item (
    id serial PRIMARY KEY,
    title varchar(150) NOT NULL,
    checked boolean NOT NULL DEFAULT false,
    list_id integer NOT NULL,
    FOREIGN KEY (list_id) REFERENCES todo_list(id)
);

INSERT INTO todo_list (title) VALUES ('List 1'), ('List 2');
INSERT INTO todo_item (title, list_id) VALUES ('item 1', 1),('item 2', 2);