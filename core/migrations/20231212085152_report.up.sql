-- Add up migration script here
create extension if not exists postgis;

create table reports (
    id          UUID primary key not null default uuid_generate_v4(),
    type_id     UUID not null,
    user_id     UUID,
    created_at  timestamp with time zone default now(),
    location    geometry,
    intensity   "char",
    -- constraints --
    constraint fk_family foreign key(type_id) references nuisance_types(id),
    constraint fk_user   foreign key(user_id) references users(id)
);

create index reports_locations on reports using GIST(location);
create index reports_creation_dates on reports(created_at);
create index reports_types on reports(type_id);