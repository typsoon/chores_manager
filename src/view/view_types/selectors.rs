use druid::Selector;
use crate::view::view_types::utils::MonthData;

pub const LOG_IN: Selector<()> = Selector::new("log-in");
#[allow(dead_code)]
const UPDATE_DATA: Selector = Selector::new("app.update-data");
#[allow(dead_code)]
const CHANGE_MONTH: Selector<MonthData> = Selector::new("app.update-month");