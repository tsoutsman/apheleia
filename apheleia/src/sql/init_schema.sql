
CREATE SCHEMA {schema};

CREATE TABLE {schema}.entity(
    id      integer PRIMARY KEY
);

CREATE TABLE {schema}.item(
    id      integer PRIMARY KEY
);

CREATE TABLE {schema}.role(
    id      integer PRIMARY KEY,
    name    varchar(255) NOT NULL
);

CREATE TABLE {schema}.archetype(
    id      integer PRIMARY KEY,
    name    varchar(255) NOT NULL
);

CREATE TABLE {schema}.entity_role(
    entity  integer REFERENCES {schema}.entity NOT NULL,
    role    integer REFERENCES {schema}.role NOT NULL,
    PRIMARY KEY (entity, role)
);

CREATE TABLE {schema}.loan(
    id              integer PRIMARY KEY,
    item            integer REFERENCES {schema}.item NOT NULL,
    loaner          integer REFERENCES {schema}.entity NOT NULL,
    loanee          integer REFERENCES {schema}.entity NOT NULL,
    date_loaned     timestamptz NOT NULL,
    date_returned   timestamptz
);

CREATE TABLE {schema}.item_archetype(
    item        integer REFERENCES {schema}.item NOT NULL,
    archetype   integer REFERENCES {schema}.archetype NOT NULL,
    PRIMARY KEY (item, archetype)
);
