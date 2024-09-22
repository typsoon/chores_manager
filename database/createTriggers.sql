SET SEARCH_PATH TO chores_manager;

CREATE OR REPLACE FUNCTION insertOneTimeChores() RETURNS TRIGGER
AS
$$
    DECLARE
        func_time_slot_id INTEGER;
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

        INSERT INTO scheduleupdates(message)
        VALUES(func_message)
        RETURNING time_slot_id INTO func_time_slot_id;

        new.time_slot_id = func_time_slot_id;
        RETURN new;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER insertOneTimeChores BEFORE INSERT ON onetimechores
    FOR EACH ROW EXECUTE PROCEDURE insertOneTimeChores();


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
                SELECT sc.time_slot_id, date_from, date_to, sc.interval FROM scheduledchores sc
            ) un WHERE un.time_slot_id = new.time_slot_id LIMIT 1) as un;

        IF bounds_and_interval IS NULL THEN
            RAISE EXCEPTION 'No such chore in database';
        END IF;

        IF bounds_and_interval.date_from + (new.iteration-1)*bounds_and_interval.chore_interval BETWEEN bounds_and_interval.date_from AND bounds_and_interval.date_to THEN
            RETURN NEW;
        ELSE
            RAISE EXCEPTION 'Wrong iteration value';
        end if;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER completeChore BEFORE INSERT ON completedchores
    FOR EACH ROW EXECUTE PROCEDURE completeChore();