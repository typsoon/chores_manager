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
