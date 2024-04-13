create database students_db;
create table city (
    id integer not null generated always as identity,
    name text not null
);
create table student (
    id integer not null generated always as identity,
    name text not null,
    city_id integer not null references city(id)
);
