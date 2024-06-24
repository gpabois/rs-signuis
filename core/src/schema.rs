diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Bool};

    /// Table utilisateurs
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        enabled -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Timestamptz};

    /// Table des sessions
    sessions(id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamptz,
        created_at -> Timestamptz
    }
}

diesel::table! {
    use diesel::sql_types::{Uuid, Varchar, Text};

    /// Table de famille de nuisances
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
diesel::allow_tables_to_appear_in_same_query!(nuisance_types, nuisance_families,);

diesel::table! {
    use diesel::sql_types::{Uuid, Nullable, Timestamptz, SmallInt};
    use postgis_diesel::sql_types::Geometry;

    nuisance_reports(id) {
        id -> Uuid,
        type_id -> Uuid,
        intensity -> SmallInt,
        location -> Geometry,
        user_id -> Nullable<Uuid>,
        created_at -> Timestamptz
    }
}

diesel::joinable!(nuisance_reports -> users(user_id));
diesel::joinable!(nuisance_reports -> nuisance_types(type_id));
diesel::allow_tables_to_appear_in_same_query!(nuisance_reports, users);

