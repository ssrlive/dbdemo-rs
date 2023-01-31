-- Your SQL goes here
create table products (
  id integer primary key autoincrement not null,
  name varchar not null,
  description varchar not null,
  create_at timestamp not null default current_timestamp
);
