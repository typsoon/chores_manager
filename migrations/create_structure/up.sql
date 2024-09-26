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
    chore_interval INTERVAL NOT NULL Check ( chore_interval >= '1 days' ),
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

-- createPrivateViews.sql
--======================

CREATE VIEW chores_manager.PeopleChoresPrivateView AS (
    SELECT * FROM peoplechores
        JOIN people USING (person_id)
        JOIN chores USING (chore_id)
);

CREATE VIEW chores_manager.ChoresNamesPrivateView AS (
    SELECT *
    FROM chores_manager.chores_manager.peoplechoresprivateview
    JOIN (
        SELECT time_slot_id, mapping_id
        FROM chores_manager.scheduledchores
        UNION
        SELECT time_slot_id, mapping_id
        FROM chores_manager.onetimechores
    ) AS scheduled
        USING (mapping_id)
);

-- createFunctions.sql
--===================

CREATE OR REPLACE FUNCTION chores_manager.getPersonName(person_id INT) RETURNS VARCHAR AS
    $$
        BEGIN
            RETURN (SELECT person_name FROM chores_manager.people p WHERE p.person_id = getPersonName.person_id LIMIT 1);
        END;
    $$ LANGUAGE plpgsql;

SET SEARCH_PATH TO chores_manager;

CREATE OR REPLACE FUNCTION wasCompleted(time_slot_id INTEGER, iteration INTEGER) RETURNS BOOLEAN AS
    $$
    BEGIN
        IF EXISTS (SELECT * FROM completedchores cc WHERE cc.time_slot_id = wasCompleted.time_slot_id AND cc.iteration = wasCompleted.iteration) THEN
            RETURN TRUE;
        ELSE
            RETURN FALSE;
        END IF;
    END;
    $$ LANGUAGE plpgsql;

CREATE TYPE chores_manager.chore_record AS (
    person_name VARCHAR,
    chore_name VARCHAR,
    date_of TIMESTAMP,
    who_updated VARCHAR,
    iteration INTEGER,
    was_completed BOOLEAN
);

CREATE OR REPLACE FUNCTION chores_manager.genChoresFromScheduled() RETURNS TABLE (
    person_name VARCHAR,
    chore_name VARCHAR,
    date_of TIMESTAMP,
    who_updated VARCHAR,
    iteration INTEGER,
    was_completed BOOLEAN
)
AS
$$
    DECLARE
        scheduled_chore record;
        answer chores_manager.chore_record[];
        act_date TIMESTAMP;
        iteration INTEGER;
    BEGIN
        FOR scheduled_chore IN SELECT * FROM scheduledchores JOIN peoplechoresprivateview USING (mapping_id) JOIN scheduleupdates USING (time_slot_id)
            LOOP
            act_date = scheduled_chore.date_from;

            iteration = 1;
            WHILE act_date <= scheduled_chore.date_to LOOP
                answer := array_append(
                        answer, (
                                 scheduled_chore.person_name, scheduled_chore.chore_name,
                                 act_date, getPersonName(scheduled_chore.who_updated),
                                 iteration,
                                 wasCompleted(scheduled_chore.time_slot_id, iteration))::chores_manager.chore_record);
                act_date := act_date + scheduled_chore.chore_interval;
                iteration := iteration+1;
            end loop;
        end loop;

        RETURN QUERY
        SELECT a.person_name, a.chore_name, a.date_of, a.who_updated, a.iteration, a.was_completed
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

        SELECT pc.mapping_id INTO mapping_id FROM PeopleChores pc WHERE pc.chore_id = func_chore_id AND pc.person_id = func_person_id;

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
    SELECT person_name, chore_name, chore_interval, date_from, date_to
    FROM scheduledchores JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
);

CREATE VIEW chores_manager.OneTimeChoresView AS (
    SELECT person_name, chore_name, onetimechores.date_of, getPersonName(who_updated) AS who_updated, 1 AS iteration, wasCompleted(time_slot_id, 1) AS was_completed
    FROM onetimechores
    JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
    JOIN scheduleupdates USING (time_slot_id)
);

CREATE VIEW chores_manager.AllChoresView AS (
    SELECT * FROM OneTimeChoresView
    UNION
    SELECT * FROM genchoresfromscheduled()
);

CREATE VIEW chores_manager.CompletedChoresView AS (
    SELECT chore_name, iteration, message FROM completedchores JOIN chores_manager.ChoresNamesPrivateView USING (time_slot_id)
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
    INSERT INTO onetimechores(time_slot_id, mapping_id, date_of)
    VALUES (null, getmapping(NEW.person_name, NEW.chore_name), NEW.date_of);
);

CREATE RULE addScheduledChores AS ON INSERT TO scheduledchoresview DO INSTEAD (
    -- noinspection SqlInsertNullIntoNotNull
    INSERT INTO scheduledchores(time_slot_id, mapping_id, chore_interval, date_from, date_to)
    VALUES (null, getmapping(NEW.person_name, NEW.chore_name), NEW.chore_interval, NEW.date_from, NEW.date_to);
);

CREATE RULE completeChore AS ON INSERT TO completedchoresview DO INSTEAD (
    INSERT INTO completedchores(time_slot_id, iteration, message)
    VALUES (
        (SELECT time_slot_id FROM chores_manager.choresnamesprivateview WHERE chore_name = new.chore_name),
        new.iteration,
        new.message
    )
);

-- createTriggers.sql
--==================

SET SEARCH_PATH TO chores_manager;

CREATE OR REPLACE FUNCTION __getPersonAndChore(mapping_id INTEGER) RETURNS RECORD
    AS $$
        DECLARE
            person_and_chore RECORD;
        BEGIN
            SELECT p.person_name, c.chore_name
            INTO person_and_chore
            FROM peoplechores pc
                     JOIN chores c ON pc.chore_id = c.chore_id
                     JOIN people p ON pc.person_id = p.person_id
            WHERE pc.mapping_id = __getPersonAndChore.mapping_id;
            RETURN person_and_chore;
        END;
    $$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION __insertMessageReturnTimeSlotId(update_message VARCHAR) RETURNS INTEGER
    AS $$
        DECLARE
            func_time_slot_id INTEGER;
        BEGIN
            INSERT INTO scheduleupdates(message)
            VALUES(update_message)
            RETURNING time_slot_id INTO func_time_slot_id;
            RETURN func_time_slot_id;
        END;
    $$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION insertOneTimeChores() RETURNS TRIGGER
AS
$$
    DECLARE
        person_and_chore RECORD = __getPersonAndChore(NEW.mapping_id);
        func_message VARCHAR;
    BEGIN
--         message = 'Added one time chore (chore_name, person_name)=(' || person_and_chore || ')';
        func_message = 'Added one time chore person_name = '
                       || person_and_chore.person_name
                       || ', chore_name = '
                       || person_and_chore.chore_name;

        new.time_slot_id = __insertMessageReturnTimeSlotId(func_message);
        RETURN new;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER insertOneTimeChores BEFORE INSERT ON onetimechores
    FOR EACH ROW EXECUTE PROCEDURE insertOneTimeChores();

CREATE OR REPLACE FUNCTION insertScheduledChores() RETURNS TRIGGER
AS
$$
    DECLARE
        person_and_chore RECORD = __getPersonAndChore(NEW.mapping_id);
        func_message VARCHAR;
    BEGIN
--         message = 'Added one time chore (chore_name, person_name)=(' || person_and_chore || ')';
        func_message = 'Added scheduled chore person_name = '
                       || person_and_chore.person_name
                       || ', chore_name = '
                       || person_and_chore.chore_name;

        new.time_slot_id = __insertMessageReturnTimeSlotId(func_message);
        RETURN new;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER insertScheduledChores BEFORE INSERT ON scheduledchores
    FOR EACH ROW EXECUTE PROCEDURE insertScheduledChores();


CREATE OR REPLACE FUNCTION completeChore() RETURNS TRIGGER
AS
$$
    DECLARE
        bounds_and_interval RECORD;
    BEGIN
        SELECT INTO bounds_and_interval
            time_slot_id, date_from, date_to, chore_interval
        FROM (SELECT * FROM
            (
                SELECT time_slot_id, date_of AS date_from, date_of AS date_to, '1 day' AS chore_interval FROM onetimechores
                UNION
                SELECT sc.time_slot_id, date_from, date_to, sc.chore_interval FROM scheduledchores sc
            ) un WHERE un.time_slot_id = new.time_slot_id LIMIT 1) as un;

        IF bounds_and_interval IS NULL THEN
            RAISE EXCEPTION 'No such chore in database';
        END IF;

        IF bounds_and_interval.date_from + (new.iteration-1)*bounds_and_interval.chore_interval BETWEEN bounds_and_interval.date_from- interval '1' hour AND bounds_and_interval.date_to+ interval '1' hour THEN
            RETURN NEW;
        ELSE
            RAISE EXCEPTION 'Wrong iteration value';
        end if;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER completeChore BEFORE INSERT ON completedchores
    FOR EACH ROW EXECUTE PROCEDURE completeChore();

