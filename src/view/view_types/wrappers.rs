use crate::model::types::{ChoreTypeRecord, FullChoreDataRecord, PersonRecord};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use druid::im::Vector;
use druid::{Data, Lens};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Data, Default)]
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

impl Display for PersonRecordWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.person_record.person_name().fmt(f)
    }
}

#[derive(Clone, Data, Default)]
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

#[derive(Clone, Data, Lens)]
pub struct FullDayData {
    people: Arc<Vec<PersonRecordWrapper>>,
    chores: Arc<Vec<ChoreTypeRecordWrapper>>,
    keyval: ChoresDataKeyVal,
}

impl FullDayData {
    pub fn new(
        people: Arc<Vec<PersonRecordWrapper>>,
        chores: Arc<Vec<ChoreTypeRecordWrapper>>,
        keyval: ChoresDataKeyVal,
    ) -> Self {
        Self {
            people,
            chores,
            keyval,
        }
    }

    pub fn get_chores(&self) -> &Arc<Vec<ChoreTypeRecordWrapper>> {
        &self.chores
    }

    pub fn get_people(&self) -> &Arc<Vec<PersonRecordWrapper>> {
        &self.people
    }

    pub fn get_day(&self) -> NaiveDateTime {
        self.keyval.day.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }
}

// pub type ImportantWeeks = Vector<Vector<ChoresDataKeyVal>>;
pub type ImportantWeeks = Vector<Vector<FullDayData>>;
