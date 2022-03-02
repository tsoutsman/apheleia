-- TODO: Add indexes

CREATE TABLE "user"(
    id      integer PRIMARY KEY
);

CREATE TABLE subject_area(
    id      integer PRIMARY KEY,
    name    varchar(255) NOT NULL,
    admin   integer REFERENCES "user" NOT NULL
);

CREATE TABLE role(
    id              integer PRIMARY KEY,
    name            varchar(255) NOT NULL,
    subject_area    integer REFERENCES subject_area NOT NULL
);

CREATE TABLE user_roles(
    "user"     integer REFERENCES "user",
    role        integer REFERENCES role,
    PRIMARY KEY ("user", role)
);

CREATE TABLE archetype(
    id              integer PRIMARY KEY,
    name            varchar(255) NOT NULL,
    subject_area    integer REFERENCES subject_area NOT NULL,
    schema          text NOT NULL
);

CREATE TABLE item(
    id              integer PRIMARY KEY,
    note            text,
    archetype       integer REFERENCES archetype NOT NULL,
    archetype_data  jsonb
);

CREATE TABLE role_permissions(
    role        integer REFERENCES role,
    archetype   integer REFERENCES "user",
    PRIMARY KEY (role, archetype),
    loan        boolean NOT NULL,
    borrow      boolean NOT NULL,
    "create"    boolean NOT NULL,
    modify      boolean NOT NULL,
    delete      boolean NOT NULL
);

CREATE TABLE loan(
    id                  integer PRIMARY KEY,
    return_requested    boolean NOT NULL,
    item                integer REFERENCES item NOT NULL,
    loaner              integer REFERENCES "user" NOT NULL,
    loanee              integer REFERENCES "user" NOT NULL,
    note                text,
    date_loaned         timestamptz NOT NULL,
    date_due            timestamptz,
    date_returned       timestamptz
);
