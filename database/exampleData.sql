INSERT INTO people(person_name) VALUES (current_user);
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore1', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore2', 'random description');
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore3', 'random description');

INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore1', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore2', current_date);
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore3', current_date);

-- INSERT INTO updates(message) VALUES ('ooo');
