use std::collections::HashMap;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{Interval, Timestamp, VarChar};

pub struct Credentials(pub String, pub String);
pub type ChoresData = HashMap<NaiveDate, Vec<FullChoreDataRecord>>;

#[allow(dead_code)]
#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PersonRecord {
    #[diesel(sql_type = VarChar)]
    pub person_name: String,
}

#[allow(dead_code)]
#[derive(QueryableByName, Debug, Eq, Hash, PartialEq)]
pub struct ChoreTypeRecord {
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = VarChar)]
    chore_description: String,
}

#[allow(dead_code)]
#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChoreRecord {
    #[diesel(sql_type = VarChar)]
    person_name: String,
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = Timestamp)]
    date_of: NaiveDateTime
}

#[allow(dead_code)]
#[derive(QueryableByName, Debug, Eq, Hash, PartialEq)]
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

#[allow(dead_code)]
impl ScheduledChoreRecord {
    pub fn get_interval(&self) -> Duration {
        // TODO: make this better
        // Duration::seconds(self.interval.parse::<i64>().unwrap())
        self.interval
    }
}

#[derive(QueryableByName, Clone, Debug, Eq, Hash, PartialEq)]
pub struct FullChoreDataRecord {
    #[diesel(sql_type = VarChar)]
    person_name: String,
    #[diesel(sql_type = VarChar)]
    chore_name: String,
    #[diesel(sql_type = Timestamp)]
    pub date_of: NaiveDateTime,
    #[diesel(sql_type = VarChar)]
    who_updated: String,
}

#[derive(Debug)]
pub enum DatabaseError {
    Error(diesel::result::Error),
}