table! {
    archetype (id) {
        id -> Uuid,
        name -> Varchar,
        subject_area -> Uuid,
        schema -> Jsonb,
    }
}

table! {
    item (id) {
        id -> Uuid,
        note -> Nullable<Text>,
        archetype -> Uuid,
        archetype_data -> Jsonb,
    }
}

table! {
    loan (id) {
        id -> Uuid,
        return_requested -> Bool,
        item -> Uuid,
        loaner -> Int4,
        loanee -> Int4,
        note -> Nullable<Text>,
        date_loaned -> Timestamptz,
        date_due -> Nullable<Timestamptz>,
        date_returned -> Nullable<Timestamptz>,
    }
}

table! {
    role (id) {
        id -> Uuid,
        name -> Varchar,
        subject_area -> Uuid,
    }
}

table! {
    role_permissions (role, archetype) {
        role -> Uuid,
        archetype -> Uuid,
        meta -> Bool,
        loan -> Bool,
        receive -> Bool,
    }
}

table! {
    subject_area (id) {
        id -> Uuid,
        name -> Varchar,
        admin -> Int4,
    }
}

table! {
    user (id) {
        id -> Int4,
    }
}

table! {
    user_roles (user, role) {
        user -> Int4,
        role -> Uuid,
    }
}

joinable!(archetype -> subject_area (subject_area));
joinable!(item -> archetype (archetype));
joinable!(loan -> item (item));
joinable!(role -> subject_area (subject_area));
joinable!(role_permissions -> archetype (archetype));
joinable!(role_permissions -> role (role));
joinable!(subject_area -> user (admin));
joinable!(user_roles -> role (role));
joinable!(user_roles -> user (user));

allow_tables_to_appear_in_same_query!(
    archetype,
    item,
    loan,
    role,
    role_permissions,
    subject_area,
    user,
    user_roles,
);
