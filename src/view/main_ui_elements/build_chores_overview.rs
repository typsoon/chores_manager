use crate::view::main_ui_elements::day_widget_grid::build_day_widget_grid;
use crate::view::utils::date_utils::MonthData;
use crate::view::utils::ui_utils::change_month_widget;
use crate::view::utils::ui_utils::change_month_widget::ReadableMonthData;
use crate::view::view_types::app_state::{DatabaseData, ImportantWeeksNewLens};
use druid::widget::Flex;
use druid::{Widget, WidgetExt};

impl ReadableMonthData for DatabaseData {
    fn get_month_data(&self) -> MonthData {
        *self.get_month_data()
    }
}

pub fn build_chores_overview() -> impl Widget<DatabaseData> {
    Flex::column()
        .with_child(change_month_widget::get_change_month_widget())
        .with_default_spacer()
        // .with_child(build_day_widget_grid().lens(ImportantWeeksLens))
        .with_child(build_day_widget_grid().lens(ImportantWeeksNewLens))
}
