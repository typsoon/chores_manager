use crate::model::traits::ReadOnlyDatabaseService;
use crate::view::main_ui_elements::calendar_widget::build_calendar_widget;
use crate::view::main_ui_elements::people_and_chores_lists::{build_list_of_chores, build_list_of_people};
use crate::view::view_types::app_state::MainStateData;
use druid::widget::TabsEdge::Leading;
use druid::widget::TabsTransition::Slide;
use druid::widget::{Axis, Flex, Label, Tabs};
use druid::{Widget, WidgetExt};
use std::rc::Rc;
use std::time::Duration;

pub fn build_main_ui_tabs(read_only_database_service: Rc<dyn ReadOnlyDatabaseService>) -> impl Widget<MainStateData> {

    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(Leading)
        .with_transition(Slide(Duration::from_millis(250).as_nanos() as u64))
        .with_tab("Chores Calendar", build_calendar_widget(read_only_database_service).lens(MainStateData::database_data))
        // .with_tab("People", build_list_of_people())
        // .with_tab("Chores", build_list_of_chores())
        .with_tab("People and chore types", Flex::row()
            .with_child(build_list_of_people())
            .with_default_spacer()
            .with_child(build_list_of_chores()))
        .with_tab("Schedule chores", Label::new("Not implemented yet"))
        .with_tab("Complete chore", Label::new("Not implemented yet"))
}