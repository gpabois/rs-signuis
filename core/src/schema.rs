use crate::fixtures::{nuisance_families, nuisance_types};

/// Table utilisateurs
diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Bool};

    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        enabled -> Bool,
    }
}

/// Table des sessions
diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Timestamptz};

    sessions(id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamptz,
        created_at -> Timestamptz
    }
}

/// Table de famille de nuisances
diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Text};

    nuisance_families(id) {
        id -> Uuid,
        label -> Varchar,
        description -> Text
    }
}

diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Text};

    nuisance_types(id) {
        id -> Uuid,
        family_id -> Uuid,
        label -> Varchar,
        description -> Text,
    }
}

diesel::joinable!(nuisance_types -> nuisance_families(family_id));
diesel::allow_tables_to_appear_in_same_query!(
    nuisance_types,
    nuisance_families,
);