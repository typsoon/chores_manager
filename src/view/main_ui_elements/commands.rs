use druid::Selector;
use crate::view::view_types::MonthData;

const UPDATE_CALENDAR: Selector = Selector::new("app.update-calendar");
const CHANGE_MONTH: Selector<MonthData> = Selector::new("app.update-month");