-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS Report (
    id          UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    type_id     UUID NOT NULL,
    user_id     UUID,
    created_at  TIMESTAMP WITH TIME ZONE,
    location    geometry,
    intensity   smallint,

    CONSTRAINT fk_family FOREIGN KEY(type_id) REFERENCES NuisanceType(id)
)

CREATE INDEX IF NOT EXISTS report_location ON Report USING GIST(location);
CREATE INDEX IF NOT EXISTS report_created_at ON Report(created_at);
CREATE INDEX IF NOT EXISTS report_type ON Report(type_id);