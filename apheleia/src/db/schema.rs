table! {
    archetype (id) {
        id -> Int4,
        name -> Varchar,
        subject_area -> Int4,
        schema -> Text,
    }
}

table! {
    item (id) {
        id -> Int4,
        note -> Nullable<Text>,
        archetype -> Int4,
        archetype_data -> Nullable<Jsonb>,
    }
}

table! {
    loan (id) {
        id -> Int4,
        return_requested -> Bool,
        item -> Int4,
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
        id -> Int4,
        name -> Varchar,
        subject_area -> Int4,
    }
}

table! {
    role_permissions (role, archetype) {
        role -> Int4,
        archetype -> Int4,
        loan -> Bool,
        borrow -> Bool,
        create -> Bool,
        modify -> Bool,
        delete -> Bool,
    }
}

table! {
    subject_area (id) {
        id -> Int4,
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
        role -> Int4,
    }
}

joinable!(archetype -> subject_area (subject_area));
joinable!(item -> archetype (archetype));
joinable!(loan -> item (item));
joinable!(role -> subject_area (subject_area));
joinable!(role_permissions -> role (role));
joinable!(role_permissions -> user (archetype));
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
