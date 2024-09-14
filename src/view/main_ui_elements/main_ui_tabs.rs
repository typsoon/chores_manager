use crate::model::types::ChoresData;
use crate::view::main_ui_elements::calendar_widget::build_calendar_widget;
use crate::view::view_types::AppState;
use druid::widget::TabsEdge::Leading;
use druid::widget::TabsTransition::Slide;
use druid::widget::{Axis, Label, Tabs};
use druid::Widget;
use std::time::Duration;

pub fn build_main_ui_tabs(chores_data: &ChoresData) -> impl Widget<AppState> {
    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(Leading)
        .with_transition(Slide(Duration::from_millis(250).as_nanos() as u64))
        .with_tab("Chores Calendar", build_calendar_widget(chores_data))
        .with_tab("People", Label::new("Not implemented yet"))
        .with_tab("Chores", Label::new("Not implemented yet"))

        // .with_tab("People2", Label::new("Not implemented yet"))
}