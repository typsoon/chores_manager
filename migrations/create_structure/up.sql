-- createTables.sql
--================

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

-- createPrivateViews.sql
--======================

CREATE VIEW chores_manager.PeopleChoresPrivateView AS (
    SELECT * FROM peoplechores
        JOIN people USING (person_id)
        JOIN chores USING (chore_id)
);


-- createFunctions.sql
--===================

CREATE TYPE chores_manager.chore_record AS (
   person_name VARCHAR,
   chore_name VARCHAR,
   date_of TIMESTAMP
);

CREATE OR REPLACE FUNCTION chores_manager.genChoresFromScheduled() RETURNS TABLE (
    person_name VARCHAR,
    chore_name VARCHAR,
    date_of TIMESTAMP
)
AS
$$
    DECLARE
        scheduled_chore record;
        answer chores_manager.chore_record[];
        act_date TIMESTAMP;
    BEGIN
        FOR scheduled_chore IN SELECT * FROM scheduledchores JOIN peoplechoresprivateview USING (mapping_id) LOOP
            act_date = scheduled_chore.date_from;

            WHILE act_date <= scheduled_chore.date_to LOOP
                answer := array_append(answer, (scheduled_chore.person_name, scheduled_chore.chore_name, act_date)::chores_manager.chore_record);
                act_date = act_date + scheduled_chore.interval;
            end loop;
        end loop;

        RETURN QUERY
        SELECT a.person_name, a.chore_name, a.date_of
        FROM UNNEST(answer) AS a;
    END;
$$
LANGUAGE plpgsql;

-- createViews.sql
--===============

CREATE VIEW chores_manager.PeopleView AS (
    SELECT person_name FROM chores_manager.People
);

CREATE VIEW chores_manager.ChoresView AS (
    SELECT chore_name, chore_description FROM chores_manager.chores
);

CREATE VIEW chores_manager.ScheduledChoresView AS (
    SELECT person_name, chore_name, interval, date_from, date_to
    FROM scheduledchores JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
);

CREATE VIEW chores_manager.OneTimeChoresView AS (
    SELECT person_name, chore_name, date_of
    FROM onetimechores JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
);

CREATE VIEW chores_manager.AllChoresView AS (
    SELECT * FROM OneTimeChoresView
    UNION
    SELECT * FROM genchoresfromscheduled()
);

