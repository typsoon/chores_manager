CREATE VIEW chores_manager.PeopleChoresPrivateView AS (
    SELECT * FROM peoplechores
        JOIN people USING (person_id)
        JOIN chores USING (chore_id)
);
