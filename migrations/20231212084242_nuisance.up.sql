-- Add up migration script here
CREATE DATABASE IF NOT EXISTS NuisanceFamily (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    label VARCHAR(255) NOT NULL,
    description TEXT
);

CREATE INDEX IF NOT EXISTS nuisance_family_label ON NuisanceFamily (label);

CREATE DATABASE IF NOT EXISTS NuisanceType (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    family_id UUID NOT NULL,
    label VARCHAR(255) NOT NULL,
    description TEXT,
    
    CONSTRAINT fk_family FOREIGN KEY(family_id) REFERENCES NuisanceFamily(id)
);

CREATE INDEX IF NOT EXISTS nuisance_type_label ON NuisanceType (label);