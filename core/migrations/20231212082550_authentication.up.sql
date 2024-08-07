-- Add up migration script here
create extension if not exists "uuid-ossp";

create table users (
    id uuid primary key not null default uuid_generate_v4(),
    username varchar(50) not null,
    email varchar(50) not null,
    password varchar(255),
    avatar varchar(255),
    email_verified_at timestamp with time zone,
    registered_at timestamp with time zone default now(),
    activated boolean not null default true,
    role varchar(50) default 'user',
    -- Settings --
    locale varchar(10) default 'fr'
);

create unique index users_unique_name on users (name);
create unique index users_unique_email on users (email);

create table sessions (
    id          uuid primary key not null default uuid_generate_v4(),
    user_id     uuid,
    token       varchar(255),
    created_at  timestamp with time zone default now(),
    expires_at  timestamp with time zone not null,
    -- Foreign key to user --
    constraint fk_users foreign key(user_id) references users(id) on delete cascade
);

create unique index sessions_unique_token on sessions (token) include (user_id, client_ip);
create index sessions_ip on sessions (client_ip);