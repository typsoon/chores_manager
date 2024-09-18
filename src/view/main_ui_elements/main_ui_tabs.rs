use crate::model::traits::ReadOnlyDatabaseService;
use crate::view::main_ui_elements::calendar_widget::build_calendar_widget;
use crate::view::main_ui_elements::people_and_chores_lists::{build_list_of_chores, build_list_of_people};
use crate::view::view_types::app_state::AppState;
use druid::widget::TabsEdge::Leading;
use druid::widget::TabsTransition::Slide;
use druid::widget::{Axis, Tabs};
use druid::Widget;
use std::rc::Rc;
use std::time::Duration;

pub fn build_main_ui_tabs(read_only_database_service: Rc<dyn ReadOnlyDatabaseService>) -> impl Widget<AppState> {
    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(Leading)
        .with_transition(Slide(Duration::from_millis(250).as_nanos() as u64))
        .with_tab("Chores Calendar", build_calendar_widget(read_only_database_service))
        .with_tab("People", build_list_of_people())
        .with_tab("Chores", build_list_of_chores())

        // .with_tab("People2", Label::new("Not implemented yet"))
}