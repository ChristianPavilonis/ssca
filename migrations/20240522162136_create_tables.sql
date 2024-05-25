-- Add migration script here

create table rooms (
	id integer primary key
	name VARCHAR
)

create table users (
	id integer primary key
	name VARCHAR
	password VARCHAR
)

-- todos:
-- - users
-- - assign users to rooms
