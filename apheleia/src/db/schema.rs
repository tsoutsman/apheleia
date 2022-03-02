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
        subject_area -> Int4,
        note -> Nullable<Text>,
        archetype -> Nullable<Int4>,
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
    role_permission (role, archetype) {
        role -> Int4,
        archetype -> Int4,
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
    user_id (id) {
        id -> Int4,
    }
}

table! {
    user_role (user_id, role) {
        user_id -> Int4,
        role -> Int4,
    }
}

joinable!(archetype -> subject_area (subject_area));
joinable!(item -> archetype (archetype));
joinable!(item -> subject_area (subject_area));
joinable!(loan -> item (item));
joinable!(role -> subject_area (subject_area));
joinable!(role_permission -> role (role));
joinable!(role_permission -> user_id (archetype));
joinable!(subject_area -> user_id (admin));
joinable!(user_role -> role (role));
joinable!(user_role -> user_id (user_id));

allow_tables_to_appear_in_same_query!(
    archetype,
    item,
    loan,
    role,
    role_permission,
    subject_area,
    user_id,
    user_role,
);
