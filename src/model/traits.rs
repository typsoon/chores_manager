use crate::model::types::*;
use chrono::NaiveDate;

pub trait ReadOnlyDatabaseService {
    fn get_chores_in_interval(
        &self,
        since: NaiveDate,
        until: NaiveDate,
    ) -> Result<ChoresData, DatabaseError>;
    fn get_people(&self) -> Result<Vec<PersonRecord>, DatabaseError>;
    fn get_chores(&self) -> Result<Vec<ChoreTypeRecord>, DatabaseError>;
}

pub trait DatabaseService: ReadOnlyDatabaseService {
    fn add_scheduled_chore(&mut self, scheduled_chore_record: ScheduledChoreRecord);
    fn add_one_time_chore(&mut self, one_time_chore_record: ChoreRecord);
    fn add_person(&mut self, person_record: PersonRecord);
    fn add_chore_type(&mut self, chore_type_record: ChoreTypeRecord);
}
