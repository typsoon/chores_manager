SET SEARCH_PATH TO chores_manager;

CREATE VIEW chores_manager.PeopleView AS (
    SELECT person_name FROM chores_manager.People
);

CREATE VIEW chores_manager.ChoresView AS (
    SELECT chore_name, chore_description FROM chores_manager.chores
);

CREATE VIEW chores_manager.ScheduledChoresView AS (
    SELECT person_name, chore_name, interval, date_from, date_to
    FROM scheduledchores JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
);

CREATE VIEW chores_manager.OneTimeChoresView AS (
    SELECT person_name, chore_name, onetimechores.date_of, getPersonName(who_updated) AS who_updated, 1 AS iteration, wasCompleted(time_slot_id, 1) AS was_completed
    FROM onetimechores
    JOIN chores_manager.PeopleChoresPrivateView USING (mapping_id)
    JOIN scheduleupdates USING (time_slot_id)
);

CREATE VIEW chores_manager.AllChoresView AS (
    SELECT * FROM OneTimeChoresView
    UNION
    SELECT * FROM genchoresfromscheduled()
);