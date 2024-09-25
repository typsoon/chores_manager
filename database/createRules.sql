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
CREATE RULE completeChore AS ON INSERT TO completedchoresview DO INSTEAD (
    INSERT INTO completedchores(time_slot_id, iteration, message)
    VALUES (
        (SELECT time_slot_id FROM chores_manager.choresnamesprivateview WHERE chore_name = new.chore_name),
        new.iteration,
        new.message
    )
);