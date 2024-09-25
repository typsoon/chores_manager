INSERT INTO people(person_name) VALUES (current_user);
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore1', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore2', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore3', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore4', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore5', 'random description');

INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore1', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore2', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore3', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore4', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore5', current_date);

-- These 2 should fail
INSERT INTO completedchoresview(chore_name, iteration, message) VALUES ('test_chore', 2, 'random');
INSERT INTO completedchoresview(chore_name, iteration, message) VALUES ('test_choreeeeeee', 1, 'random');
-- INSERT INTO completedchores(time_slot_id, iteration, message) VALUES (1000, 1, 'random');

INSERT INTO completedchores(time_slot_id, iteration, message) VALUES (1, 1, 'random');
INSERT INTO completedchoresview(chore_name, iteration, message) VALUES ('test_chore', 1, 'random');

INSERT INTO completedchores(time_slot_id, iteration, message) VALUES (2, 1, 'random');
DELETE FROM completedchores WHERE TRUE;