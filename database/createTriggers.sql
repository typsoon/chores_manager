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