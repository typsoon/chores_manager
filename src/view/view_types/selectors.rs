use crate::model::types::PersonRecord;
use crate::model::types::{ChoreTypeRecord, CompletedChoreData};
use crate::model::types::{OneTimeChoreRecord, ScheduledChoreRecord};
use crate::view::view_types::utils::MonthData;
use druid::Selector;

pub const LOG_IN: Selector = Selector::new("log-in");

// const UPDATE_DATA: Selector = Selector::new("app.update-data");

pub const CHANGE_MONTH: Selector<MonthData> = Selector::new("app.update-month");

pub const ADD_PERSON: Selector<PersonRecord> = Selector::new("app.add-person");

pub const ADD_CHORE_TYPE: Selector<ChoreTypeRecord> = Selector::new("app.add-chore-type");

pub const COMPLETE_CHORE: Selector<CompletedChoreData> = Selector::new("app.complete-chore");

#[allow(dead_code)]
pub const ADD_ONE_TIME_CHORE: Selector<OneTimeChoreRecord> =
    Selector::new("app.add-one-time-chore");

#[allow(dead_code)]
pub const ADD_SCHEDULED_CHORE: Selector<ScheduledChoreRecord> =
    Selector::new("app.add-scheduled-chore");
