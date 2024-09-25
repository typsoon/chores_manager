CREATE VIEW chores_manager.PeopleChoresPrivateView AS (
    SELECT * FROM peoplechores
        JOIN people USING (person_id)
        JOIN chores USING (chore_id)
);

CREATE VIEW chores_manager.ChoresNamesPrivateView AS (
    SELECT *
    FROM chores_manager.chores_manager.peoplechoresprivateview
    JOIN (
        SELECT time_slot_id, mapping_id
        FROM chores_manager.scheduledchores
        UNION
        SELECT time_slot_id, mapping_id
        FROM chores_manager.onetimechores
    ) AS scheduled
        USING (mapping_id)
);