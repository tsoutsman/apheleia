SET timezone = 'Australia/ACT Australia/Canberra Australia/NSW Australia/Sydney';

CREATE SCHEMA {0};

CREATE TABLE {0}.users(
    id      integer PRIMARY KEY,
);

CREATE TABLE {0}.items(
    id      integer PRIMARY KEY,
);

CREATE TABLE {0}.roles(
    id      integer PRIMARY KEY,
    name    varchar(255) NOT NULL,
);

CREATE TABLE {0}.archetypes(
    id      integer PRIMARY KEY,
    name    varchar(255) NOT NULL,
);

CREATE TABLE {0}.user_roles(
    user    integer REFERENCES users NOT NULL,
    role    integer REFERENCES roles NOT NULL,
    PRIMARY KEY (user, role),
);

CREATE TABLE {0}.loans(
    id              integer PRIMARY KEY,
    item            integer REFERENCES items NOT NULL,
    loaner          integer REFERENCES users NOT NULL,
    loanee          integer REFERENCES users NOT NULL,
    date_loaned     timestamptz NOT NULL,
    date_returned   timestamptz,
);

CREATE TABLE {0}.item_archetypes(
    item        integer REFERENCES items NOT NULL,
    archetype   integer REFERENCES archetypes NOT NULL,
    PRIMARY KEY (item, archetype),
);
