use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{VarChar};

#[derive(QueryableByName, Debug)]
pub struct PersonRecord {
    #[diesel(sql_type = VarChar)]
    pub person_name: String,
}

#[derive(Queryable, Debug)]
struct ChoreTypeRecord {
    chore_name: String,
    chore_description: String,
}

#[derive(Queryable, Debug)]
struct ChoreRecord {
    person_name: String,
    chore_name: String,
    date_of: NaiveDateTime
}

#[derive(Queryable, Debug)]
struct ScheduledChoreRecord {
    person_name: String,
    chore_name: String,
    interval_as_str: String,
    date_from: NaiveDateTime,
    date_to: NaiveDateTime,
}

impl ScheduledChoreRecord {
    pub fn get_interval(&self) -> Duration {
        // TODO: make this better
        Duration::seconds(self.interval_as_str.parse::<i64>().unwrap())
    }
}