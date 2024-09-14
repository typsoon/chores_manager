-- createTables.sql
--================

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

CREATE TABLE chores_manager.Updates (
    update_id SERIAL PRIMARY KEY ,
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

SET SEARCH_PATH TO chores_manager;

CREATE OR REPLACE FUNCTION getPersonName(person_id INT) RETURNS VARCHAR AS
    $$
        BEGIN
            RETURN (SELECT person_name FROM people p WHERE p.person_id = getPersonName.person_id LIMIT 1);
        END;
    $$ LANGUAGE plpgsql;

CREATE TYPE chores_manager.chore_record AS (
    person_name VARCHAR,
    chore_name VARCHAR,
    date_of TIMESTAMP,
    who_updated VARCHAR
);

CREATE OR REPLACE FUNCTION chores_manager.genChoresFromScheduled() RETURNS TABLE (
    person_name VARCHAR,
    chore_name VARCHAR,
    date_of TIMESTAMP,
    who_updated VARCHAR
)
AS
$$
    DECLARE
        scheduled_chore record;
        answer chores_manager.chore_record[];
        act_date TIMESTAMP;
    BEGIN
        FOR scheduled_chore IN SELECT * FROM scheduledchores JOIN peoplechoresprivateview USING (mapping_id) JOIN updates USING (update_id)
            LOOP
            act_date = scheduled_chore.date_from;

            WHILE act_date <= scheduled_chore.date_to LOOP
                answer := array_append(answer, (scheduled_chore.person_name, scheduled_chore.chore_name, act_date, getPersonName(scheduled_chore.who_updated))::chores_manager.chore_record);
                act_date = act_date + scheduled_chore.interval;
            end loop;
        end loop;

        RETURN QUERY
        SELECT a.person_name, a.chore_name, a.date_of, a.who_updated
        FROM UNNEST(answer) AS a;
    END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION getMapping(person_name varchar, chore_name varchar) RETURNS INTEGER
AS
$$
    DECLARE
        func_person_id INTEGER;
        func_chore_id INTEGER;
        mapping_id INTEGER;
    BEGIN
        SELECT p.person_id INTO func_person_id FROM People p WHERE p.person_name = getMapping.person_name;
        SELECT c.chore_id INTO func_chore_id FROM Chores c WHERE c.chore_name = getMapping.chore_name;

        IF func_person_id IS NULL THEN
            RAISE EXCEPTION 'No such person in database';
        END IF;

        IF func_chore_id IS NULL THEN
            RAISE EXCEPTION 'No such chore in database';
        END IF;

        SELECT pc.mapping_id INTO mapping_id FROM PeopleChores pc WHERE (pc.chore_id, pc.person_id) = (func_person_id, func_chore_id);

        IF mapping_id IS NULL THEN
            INSERT INTO PeopleChores(person_id, chore_id) VALUES (func_person_id, func_chore_id) RETURNING PeopleChores.mapping_id INTO mapping_id;
        END IF;

        RETURN mapping_id;
    END;
$$
LANGUAGE plpgsql;

-- createViews.sql
--===============

SET SEARCH_PATH TO chores_manager;

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
    SELECT person_name, chore_name, onetimechores.date_of, getPersonName(who_updated) AS who_updated
    FROM onetimechores
    JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
    JOIN updates USING (update_id)
);

CREATE VIEW chores_manager.AllChoresView AS (
    SELECT * FROM OneTimeChoresView
    UNION
    SELECT * FROM genchoresfromscheduled()
);

-- createRules.sql
--===============

SET SEARCH_PATH TO chores_manager;

CREATE RULE addPeople AS ON INSERT TO peopleview DO INSTEAD (
    INSERT INTO people(person_name) VALUES (NEW.person_name)
);

CREATE RULE addChores AS ON INSERT TO choresview DO INSTEAD (
    INSERT INTO chores(chore_name, chore_description) VALUES (NEW.chore_name, NEW.chore_description)
);

CREATE RULE addOneTimeChores AS ON INSERT TO onetimechoresview DO INSTEAD (
    -- noinspection SqlInsertNullIntoNotNull
    INSERT INTO onetimechores(update_id, mapping_id, date_of) VALUES (null, getmapping(new.person_name, new.chore_name), new.date_of);
);


-- createTriggers.sql
--==================

SET SEARCH_PATH TO chores_manager;

CREATE OR REPLACE FUNCTION insertOneTimeChores() RETURNS TRIGGER
AS
$$
    DECLARE
        func_update_id INTEGER;
        person_and_chore RECORD;
        func_message VARCHAR;
    BEGIN
        SELECT p.person_name, c.chore_name
        INTO person_and_chore
        FROM peoplechores pc
                 JOIN chores c ON pc.chore_id = c.chore_id
                 JOIN people p ON pc.person_id = p.person_id
        WHERE pc.mapping_id = NEW.mapping_id;

--         message = 'Added one time chore (chore_name, person_name)=(' || person_and_chore || ')';
        func_message = 'Added one time chore person_name = '
                       || person_and_chore.person_name
                       || ', chore_name = '
                       || person_and_chore.chore_name;

        INSERT INTO updates(message)
        VALUES(func_message)
        RETURNING update_id INTO func_update_id;

        new.update_id = func_update_id;
        RETURN new;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER insertOneTimeChores BEFORE INSERT ON onetimechores
    FOR EACH ROW EXECUTE PROCEDURE insertOneTimeChores();

