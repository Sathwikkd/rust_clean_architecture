-- Your SQL goes here
create table students(
    id serial primary key,
    name varchar(20) not null,
    email varchar(20) not null,
    password varchar(20) not null
);

CREATE INDEX index_students_name ON students(name)