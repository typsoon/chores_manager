use crate::model::types::{ChoreTypeRecord, FullChoreDataRecord, PersonRecord};
use chrono::NaiveDate;
use druid::im::Vector;
use druid::{Data, Lens};
use std::ops::Deref;

#[derive(Clone, Data)]
pub struct PersonRecordWrapper {
    #[data(eq)]
    person_record: PersonRecord,
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
    chore_record: ChoreTypeRecord,
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

#[derive(Clone, Data)]
pub struct FullChoreDataWrapper {
    #[data(eq)]
    full_chore_data_record: FullChoreDataRecord,
}

impl FullChoreDataWrapper {
    pub fn new(full_chore_data_record: FullChoreDataRecord) -> Self {
        Self {
            full_chore_data_record,
        }
    }
}

impl Deref for FullChoreDataWrapper {
    type Target = FullChoreDataRecord;

    fn deref(&self) -> &Self::Target {
        &self.full_chore_data_record
    }
}

#[derive(Clone, Data)]
pub struct NaiveDateWrapper {
    #[data(eq)]
    date: NaiveDate,
}

impl NaiveDateWrapper {
    pub fn new(date: NaiveDate) -> Self {
        Self { date }
    }
}

impl Deref for NaiveDateWrapper {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.date
    }
}

impl From<NaiveDate> for NaiveDateWrapper {
    fn from(value: NaiveDate) -> Self {
        NaiveDateWrapper { date: value }
    }
}

#[derive(Clone, Data, Lens)]
pub struct ChoresDataKeyVal {
    day: NaiveDateWrapper,
    chores: Vector<FullChoreDataWrapper>,
    month: u32,
}

impl ChoresDataKeyVal {
    pub fn get_day(&self) -> &NaiveDateWrapper {
        &self.day
    }

    // pub fn get_chores(&self) -> &Vector<FullChoreDataWrapper> {
    //     &self.chores
    // }

    pub fn new(day: NaiveDateWrapper, chores: Vector<FullChoreDataWrapper>, month: u32) -> Self {
        Self { day, chores, month }
    }

    pub fn get_month(&self) -> u32 {
        self.month
    }
}

pub type ImportantWeeks = Vector<Vector<ChoresDataKeyVal>>;
