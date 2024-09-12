use std::collections::HashMap;
use chrono::NaiveDate;
use crate::model::types::*;

pub trait ReadOnlyDatabaseService {
    fn get_chores_in_interval(&mut self, since: NaiveDate, until: NaiveDate) -> Result<HashMap<NaiveDate, Vec<FullChoreDataRecord>>, ()>;
}

pub trait DatabaseService : ReadOnlyDatabaseService {
    fn add_scheduled_chore(&self, scheduled_chore_record: ScheduledChoreRecord);
    fn add_one_time_chore(&self, one_time_chore_record: ChoreRecord);
    fn add_person(&self, person_record: PersonRecord);
    fn add_chore(&self, chore_record: ChoreRecord);
}