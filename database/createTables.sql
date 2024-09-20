CREATE SCHEMA chores_manager;

SET SEARCH_PATH TO chores_manager;

CREATE TABLE chores_manager.People (
    person_id SERIAL PRIMARY KEY ,
    person_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE chores_manager.Chores (
    chore_id SERIAL PRIMARY KEY ,
    chore_name VARCHAR UNIQUE NOT NULL ,
    chore_description VARCHAR
);

CREATE OR REPLACE FUNCTION get_person_id(person_name varchar) RETURNS INTEGER AS
    $$BEGIN RETURN  (SELECT p.person_id FROM people p WHERE p.person_name = get_person_id.person_name); END$$ LANGUAGE plpgsql;

CREATE TABLE chores_manager.ScheduleUpdates (
    time_slot_id SERIAL PRIMARY KEY ,
    who_updated INTEGER NOT NULL REFERENCES chores_manager.people DEFAULT get_person_id(current_user::varchar),
    message VARCHAR NOT NULL ,
    date_of TIMESTAMP NOT NULL DEFAULT localtimestamp
);

CREATE TABLE chores_manager.PeopleChores (
    mapping_id SERIAL PRIMARY KEY ,
    person_id INTEGER NOT NULL REFERENCES chores_manager.people ,
    chore_id INTEGER NOT NULL REFERENCES chores_manager.chores ,
    UNIQUE(person_id, chore_id)
);

CREATE TABLE chores_manager.CompletedChores (
    time_slot_id INTEGER NOT NULL REFERENCES ScheduleUpdates,
    iteration INTEGER NOT NULL CHECK ( iteration > 0 ),
    who_added INTEGER NOT NULL REFERENCES People DEFAULT get_person_id(current_user::varchar),
    message VARCHAR NOT NULL ,
    date_of TIMESTAMP NOT NULL DEFAULT localtimestamp,
    PRIMARY KEY (time_slot_id, iteration)
);

CREATE TABLE chores_manager.ScheduledChores (
    time_slot_id INTEGER NOT NULL REFERENCES chores_manager.ScheduleUpdates ,
    mapping_id INTEGER NOT NULL REFERENCES PeopleChores ,
    interval INTERVAL NOT NULL ,
    date_from TIMESTAMP NOT NULL ,
    date_to TIMESTAMP NOT NULL ,
    CHECK ( date_from < date_to )
);

CREATE TABLE OneTimeChores (
    time_slot_id INTEGER NOT NULL REFERENCES chores_manager.ScheduleUpdates ,
    mapping_id INTEGER NOT NULL REFERENCES PeopleChores ,
    date_of TIMESTAMP NOT NULL
);
-- TODO: make diagram more readable