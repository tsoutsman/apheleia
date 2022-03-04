-- TODO: Add indexes

CREATE TABLE "user"(
    id      integer PRIMARY KEY
);

CREATE TABLE subject_area(
    id      uuid PRIMARY KEY,
    name    varchar(255) NOT NULL,
    admin   integer REFERENCES "user" NOT NULL
);

CREATE TABLE role(
    id              uuid PRIMARY KEY,
    name            varchar(255) NOT NULL,
    subject_area    uuid REFERENCES subject_area NOT NULL
);

CREATE TABLE user_roles(
    "user"      integer REFERENCES "user",
    role        uuid REFERENCES role,
    PRIMARY KEY ("user", role)
);

CREATE TABLE archetype(
    id              uuid PRIMARY KEY,
    name            varchar(255) NOT NULL,
    subject_area    uuid REFERENCES subject_area NOT NULL,
    schema          jsonb NOT NULL
);

CREATE TABLE item(
    id              uuid PRIMARY KEY,
    note            text,
    archetype       uuid REFERENCES archetype NOT NULL,
    archetype_data  jsonb NOT NULL
);

CREATE TABLE role_permissions(
    role        uuid REFERENCES role,
    archetype   uuid REFERENCES archetype,
    PRIMARY KEY (role, archetype),
    meta        boolean NOT NULL,
    loan        boolean NOT NULL,
    receive     boolean NOT NULL
);

CREATE TABLE loan(
    id                  uuid PRIMARY KEY,
    return_requested    boolean NOT NULL,
    item                uuid REFERENCES item NOT NULL,
    loaner              integer REFERENCES "user" NOT NULL,
    loanee              integer REFERENCES "user" NOT NULL,
    note                text,
    date_loaned         timestamptz NOT NULL,
    date_due            timestamptz,
    date_returned       timestamptz
);
