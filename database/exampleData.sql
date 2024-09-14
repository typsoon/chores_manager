INSERT INTO people(person_name) VALUES (current_user);
INSERT INTO chores(chore_name, chore_description) VALUES ('test_chore', 'random description');
INSERT INTO onetimechoresview(person_name, chore_name, date_of) VALUES (current_user, 'test_chore', current_date);

SELECT
-- INSERT INTO updates(message) VALUES ('ooo');
