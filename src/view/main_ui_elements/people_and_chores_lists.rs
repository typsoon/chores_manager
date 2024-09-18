use crate::view::view_types::app_state::AppState;
use druid::widget::{Flex, Label, List, Scroll};
use druid::{Widget, WidgetExt};
use crate::view::view_types::wrappers::{ChoreTypeRecordWrapper, PersonRecordWrapper};

const ITEM_LIST_WIDTH: f64 = 200.;

pub fn build_list_of_people() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("People"))
        .with_child(
            Scroll::new(List::new(|| Label::new(|item: &PersonRecordWrapper, _env: &_| item.person_name.clone()).padding((0.,10.,0.,0.)))
            )
                .vertical()
                .lens(AppState::people)
        )
        .fix_width(ITEM_LIST_WIDTH)
}


pub fn build_list_of_chores() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Chores"))
        .with_child(
            Scroll::new(List::new(|| Label::new(|item: &ChoreTypeRecordWrapper, _env: &_| format!("{}\n{}", item.chore_name.clone(), item.chore_description.clone())).padding((0.,10.,0.,0.)))
            )
                .vertical()
                .lens(AppState::chores)
        )
        .fix_width(ITEM_LIST_WIDTH)
}


