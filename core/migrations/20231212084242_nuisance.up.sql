-- Add up migration script here
create table nuisance_families (
    id uuid primary key not null default uuid_generate_v4(),
    label varchar(255) not null,
    description text
);

create unique index nuisance_families_labels on nuisance_families (label);

create table nuisance_types (
    id uuid primary key not null default uuid_generate_v4(),
    family_id uuid not null,
    label varchar(255) not null,
    description text,
    -- constraints --
    constraint fk_family foreign key(family_id) references nuisance_families(id)
);

create unique index nuisance_types_labels on nuisance_types (family_id, label);