use chrono::{Duration, NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Interval, Timestamp, VarChar};
use std::collections::HashMap;

pub struct Credentials(pub String, pub String);
pub type ChoresData = HashMap<NaiveDate, Vec<FullChoreDataRecord>>;

#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PersonRecord {
    #[diesel(sql_type = VarChar)]
    person_name: String,
}

impl PersonRecord {
    pub fn new(person_name: String) -> Self {
        Self { person_name }
    }

    pub fn person_name(&self) -> &str {
        &self.person_name
    }
}

#[derive(QueryableByName, Debug, Eq, Hash, PartialEq, Clone)]
pub struct ChoreTypeRecord {
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = VarChar)]
    chore_description: String,
}

impl ChoreTypeRecord {
    pub fn new(chore_name: String, chore_description: String) -> Self {
        Self {
            chore_name,
            chore_description,
        }
    }

    pub fn chore_name(&self) -> &str {
        &self.chore_name
    }

    pub fn chore_description(&self) -> &str {
        &self.chore_description
    }
}

#[allow(dead_code)]
#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq)]
pub struct OneTimeChoreRecord {
    #[diesel(sql_type = VarChar)]
    person_name: String,
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = Timestamp)]
    date_of: NaiveDateTime,
}

impl OneTimeChoreRecord {
    pub fn new(person_name: String, chore_name: String, date_of: NaiveDateTime) -> Self {
        Self {
            person_name,
            chore_name,
            date_of,
        }
    }

    pub fn person_name(&self) -> &str {
        &self.person_name
    }

    pub fn chore_name(&self) -> &str {
        &self.chore_name
    }

    pub fn date_of(&self) -> NaiveDateTime {
        self.date_of
    }
}

#[allow(dead_code)]
#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct ScheduledChoreRecord {
    #[diesel(sql_type = VarChar)]
    person_name: String,
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = Interval)]
    interval: Duration,
    #[diesel(sql_type = Timestamp)]
    date_from: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    date_to: NaiveDateTime,
}

impl ScheduledChoreRecord {
    pub fn new(
        person_name: String,
        chore_name: String,
        interval: Duration,
        date_from: NaiveDateTime,
        date_to: NaiveDateTime,
    ) -> Self {
        Self {
            person_name,
            chore_name,
            interval,
            date_from,
            date_to,
        }
    }

    pub fn person_name(&self) -> &str {
        &self.person_name
    }

    pub fn chore_name(&self) -> &str {
        &self.chore_name
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn date_from(&self) -> NaiveDateTime {
        self.date_from
    }

    pub fn date_to(&self) -> NaiveDateTime {
        self.date_to
    }
}

#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct FullChoreDataRecord {
    #[diesel(sql_type = VarChar)]
    person_name: String,
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    // #[diesel(sql_type = VarChar)]
    // chore_description: String,
    #[diesel(sql_type = Timestamp)]
    date_of: NaiveDateTime,
    #[diesel(sql_type = VarChar)]
    who_updated: String,
    #[diesel(sql_type = Integer)]
    iteration: i32,
    #[diesel(sql_type = Bool)]
    was_completed: bool,
}

impl FullChoreDataRecord {
    pub fn person_name(&self) -> &str {
        &self.person_name
    }

    pub fn chore_name(&self) -> &str {
        &self.chore_name
    }

    pub fn date_of(&self) -> NaiveDateTime {
        self.date_of
    }

    // pub fn who_updated(&self) -> &str {
    //     &self.who_updated
    // }

    pub fn was_completed(&self) -> bool {
        self.was_completed
    }

    pub fn iteration(&self) -> i32 {
        self.iteration
    }
}

#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct CompletedChoreData {
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = Integer)]
    iteration: i32,
    #[diesel(sql_type = VarChar)]
    message: String,
}

impl CompletedChoreData {
    pub fn new(chore_name: String, iteration: i32, message: String) -> Self {
        Self {
            chore_name,
            iteration,
            message,
        }
    }

    pub fn chore_name(&self) -> &str {
        &self.chore_name
    }

    pub fn iteration(&self) -> i32 {
        self.iteration
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    Error(diesel::result::Error),
}
