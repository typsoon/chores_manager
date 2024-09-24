use crate::model::types::ChoreTypeRecord;
use crate::model::types::PersonRecord;
use crate::view::view_types::utils::MonthData;
use druid::Selector;

pub const LOG_IN: Selector = Selector::new("log-in");
#[allow(dead_code)]
const UPDATE_DATA: Selector = Selector::new("app.update-data");
#[allow(dead_code)]
pub const CHANGE_MONTH: Selector<MonthData> = Selector::new("app.update-month");

#[allow(dead_code)]
pub const ADD_PERSON: Selector<PersonRecord> = Selector::new("app.add-person");

#[allow(dead_code)]
pub const ADD_CHORE_TYPE: Selector<ChoreTypeRecord> = Selector::new("app.add-chore-type");
