-- TODO: Add indexes

CREATE TABLE user_id(
    id      integer PRIMARY KEY
);

CREATE TABLE subject_area(
    id      integer PRIMARY KEY,
    name    varchar(255) NOT NULL,
    admin   integer REFERENCES user_id NOT NULL
);

CREATE TABLE role(
    id              integer PRIMARY KEY,
    name            varchar(255) NOT NULL,
    subject_area    integer REFERENCES subject_area NOT NULL
);

CREATE TABLE user_role(
    user_id     integer REFERENCES user_id,
    role        integer REFERENCES role,
    PRIMARY KEY (user_id, role)
);

CREATE TABLE archetype(
    id              integer PRIMARY KEY,
    name            varchar(255) NOT NULL,
    subject_area    integer REFERENCES subject_area NOT NULL,
    schema          text NOT NULL
);

CREATE TABLE item(
    id              integer PRIMARY KEY,
    subject_area    integer REFERENCES subject_area NOT NULL,
    note            text,
    archetype       integer REFERENCES archetype,
    archetype_data  bytea
);

CREATE TABLE role_permission(
    role        integer REFERENCES role,
    archetype   integer REFERENCES user_id,
    PRIMARY KEY (role, archetype)
    -- TODO: Add granular permissions
);

CREATE TABLE loan(
    id                  integer PRIMARY KEY,
    return_requested    boolean NOT NULL,
    item                integer REFERENCES item NOT NULL,
    loaner              integer REFERENCES user_id NOT NULL,
    loanee              integer REFERENCES user_id NOT NULL,
    note                text,
    date_loaned         timestamptz NOT NULL,
    date_due            timestamptz,
    date_returned       timestamptz
);
