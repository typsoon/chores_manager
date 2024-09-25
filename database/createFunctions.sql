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

        SELECT pc.mapping_id INTO mapping_id FROM PeopleChores pc WHERE (pc.chore_id, pc.person_id) = (func_person_id, func_chore_id);

        IF mapping_id IS NULL THEN
            INSERT INTO PeopleChores(person_id, chore_id) VALUES (func_person_id, func_chore_id) RETURNING PeopleChores.mapping_id INTO mapping_id;
        END IF;

        RETURN mapping_id;
    END;
$$
LANGUAGE plpgsql;