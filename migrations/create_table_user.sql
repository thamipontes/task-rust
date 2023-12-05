create table user (
    "id" bigserial primary key,
    "created_at" timestamptz(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamptz(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "email" varchar(50) NOT NULL,
    "password" varchar(50) NOT NULL 
);