use crate::view::view_types::app_state::{DatabaseData, MainStateData, MainStateInputData};
use crate::view::view_types::wrappers::{ChoreTypeRecordWrapper, PersonRecordWrapper};
use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{Widget, WidgetExt};

const ITEM_LIST_WIDTH: f64 = 200.;

pub fn build_list_of_people() -> impl Widget<MainStateData> {
    Flex::column()
        .with_child(Label::new("People"))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| Label::new(|item: &PersonRecordWrapper, _env: &_| item.person_name.clone()).padding((0., 10., 0., 0.)))
                    .lens(DatabaseData::people)
            )
            .vertical()
            .lens(MainStateData::database_data)
        )
        .with_child(Flex::row()
            .with_child(Button::new("Add person"))
            .with_default_spacer()
            .with_child(TextBox::new().with_placeholder("Person name").lens(MainStateInputData::added_person_name))
            .lens(MainStateData::input_data)
        )
        .fix_width(ITEM_LIST_WIDTH)
}


pub fn build_list_of_chores() -> impl Widget<MainStateData> {
    Flex::column()
        .with_child(Label::new("Chores"))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| Label::new(|item: &ChoreTypeRecordWrapper, _env: &_| format!("{}\n{}", item.chore_name.clone(), item.chore_description.clone())).padding((0.,10.,0.,0.)))
                    .lens(DatabaseData::chores)
            )
            .vertical()
            .lens(MainStateData::database_data)
        )
        .fix_width(ITEM_LIST_WIDTH)
}


