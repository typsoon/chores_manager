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