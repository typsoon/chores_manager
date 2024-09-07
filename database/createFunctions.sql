CREATE TYPE chores_manager.chore_record AS (
   person_name VARCHAR,
   chore_name VARCHAR,
   date_of TIMESTAMP
);

CREATE OR REPLACE FUNCTION chores_manager.genChoresFromScheduled() RETURNS TABLE (
    person_name VARCHAR,
    chore_name VARCHAR,
    date_of TIMESTAMP
)
AS
$$
    DECLARE
        scheduled_chore record;
        answer chores_manager.chore_record[];
        act_date TIMESTAMP;
    BEGIN
        FOR scheduled_chore IN SELECT * FROM scheduledchores JOIN peoplechoresprivateview USING (mapping_id) LOOP
            act_date = scheduled_chore.date_from;

            WHILE act_date <= scheduled_chore.date_to LOOP
                answer := array_append(answer, (scheduled_chore.person_name, scheduled_chore.chore_name, act_date)::chores_manager.chore_record);
                act_date = act_date + scheduled_chore.interval;
            end loop;
        end loop;

        RETURN QUERY
        SELECT a.person_name, a.chore_name, a.date_of
        FROM UNNEST(answer) AS a;
    END;
$$
LANGUAGE plpgsql;