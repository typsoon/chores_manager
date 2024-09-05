CREATE SCHEMA chores_manager;

CREATE TABLE chores_manager.People (
    person_id SERIAL PRIMARY KEY ,
    person_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE chores_manager.Chores (
    chore_id SERIAL PRIMARY KEY ,
    chore_name VARCHAR UNIQUE NOT NULL ,
    chore_description VARCHAR
);

CREATE TABLE chores_manager.Updates (
    update_id SERIAL PRIMARY KEY ,
    who_updated INTEGER NOT NULL REFERENCES chores_manager.people ,
    message VARCHAR
);

CREATE TABLE chores_manager.PeopleChores (
    mapping_id SERIAL PRIMARY KEY ,
    person_id INTEGER NOT NULL REFERENCES chores_manager.people ,
    chore_id INTEGER NOT NULL REFERENCES chores_manager.chores ,
    UNIQUE(person_id, chore_id)
);

CREATE TABLE chores_manager.CompletedChores (
    mapping_id INTEGER NOT NULL REFERENCES PeopleChores,
    update_id INTEGER NOT NULL REFERENCES chores_manager.updates ,
    date_of TIMESTAMP
);

CREATE TABLE chores_manager.ScheduledChores (
    update_id INTEGER NOT NULL REFERENCES chores_manager.Updates ,
    mapping_id INTEGER NOT NULL REFERENCES PeopleChores ,
    interval INTERVAL NOT NULL ,
    date_from TIMESTAMP NOT NULL ,
    date_to TIMESTAMP NOT NULL ,
    CHECK ( date_from < date_to )
);

CREATE TABLE OneTimeChores (
    update_id INTEGER NOT NULL REFERENCES chores_manager.Updates ,
    mapping_id INTEGER NOT NULL REFERENCES PeopleChores ,
    date_of TIMESTAMP NOT NULL
);
-- TODO: make diagram more readable