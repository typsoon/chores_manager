use std::ops::Deref;
use crate::model::types::{ChoreTypeRecord, PersonRecord};
use druid::Data;

#[derive(Clone, Data)]
pub struct PersonRecordWrapper {
    #[data(eq)]
    person_record: PersonRecord
}

impl PersonRecordWrapper {
    pub fn new(person_record: PersonRecord) -> Self {
        Self { person_record }
    }
}

impl Deref for PersonRecordWrapper {
    type Target = PersonRecord;

    fn deref(&self) -> &Self::Target {
        &self.person_record
    }
}

#[derive(Clone, Data)]
pub struct ChoreTypeRecordWrapper {
    #[data(eq)]
    chore_record: ChoreTypeRecord
}

impl ChoreTypeRecordWrapper {
    pub fn new(chore_record: ChoreTypeRecord) -> Self {
        Self { chore_record }
    }
}

impl Deref for ChoreTypeRecordWrapper {
    type Target = ChoreTypeRecord;

    fn deref(&self) -> &Self::Target {
        &self.chore_record
    }
}