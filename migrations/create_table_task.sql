-- Add migration script here
create table task (
    "id" varchar(36) primary key,
    "created_at" timestamptz(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamptz(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" varchar(50) NOT NULL
);