-- Add up migration script here
create table logs (
    id          UUID primary key not null default uuid_generate_v4(),
    type        varchar(255) not null,
    ------------
    -- client --
    client_ip          varchar(39) not null,
    client_user_agent  varchar(255) not null,
    ------------
    args        text,
    message     text,
    user_id     UUID,
    at          timestamp with time zone default now(),
    -- constraints --
    constraint fk_user   foreign key(user_id) references users(id) on delete set null
);

create index logs_client_ip on logs (client_ip);
