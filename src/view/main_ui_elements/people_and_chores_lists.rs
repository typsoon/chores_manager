use crate::model::types::{ChoreTypeRecord, PersonRecord};
use crate::view::view_types::app_state::{DatabaseData, MainStateData, MainStateInputData};
use crate::view::view_types::selectors::{ADD_CHORE_TYPE, ADD_PERSON};
use crate::view::view_types::wrappers::{ChoreTypeRecordWrapper, PersonRecordWrapper};
use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{Widget, WidgetExt};

const ITEM_LIST_WIDTH: f64 = 750.;
const PERSON_NAME_FIELD_WIDTH: f64 = 200.;
const CHORE_DESCRIPTION_WIDTH: f64 = 400.;

pub fn build_list_of_people() -> impl Widget<MainStateData> {
    Flex::column()
        .with_child(Label::new("People"))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| {
                    Label::new(|item: &PersonRecordWrapper, _env: &_| {
                        item.person_name().to_string()
                    })
                    .padding((0., 10., 0., 0.))
                })
                .lens(DatabaseData::people),
            )
            .vertical()
            .lens(MainStateData::database_data),
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(Button::new("Add person").on_click(
                    |ctx, data: &mut MainStateInputData, _| {
                        ctx.submit_command(
                            ADD_PERSON
                                .with(PersonRecord::new(data.get_added_person_name().to_string())),
                        )
                    },
                ))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_placeholder("Person name")
                        .fix_width(PERSON_NAME_FIELD_WIDTH)
                        .lens(MainStateInputData::added_person_name),
                )
                .expand_width()
                .lens(MainStateData::input_data),
        )
        .fix_width(ITEM_LIST_WIDTH)
}

pub fn build_list_of_chores() -> impl Widget<MainStateData> {
    Flex::column()
        .with_child(Label::new("Chores"))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| {
                    Label::new(|item: &ChoreTypeRecordWrapper, _env: &_| {
                        format!("{}\n{}", item.chore_name(), item.chore_description())
                    })
                    .padding((0., 10., 0., 0.))
                })
                .lens(DatabaseData::chores),
            )
            .vertical()
            .lens(MainStateData::database_data),
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(Button::new("Add chore type").on_click(
                    |ctx, data: &mut MainStateInputData, _| {
                        ctx.submit_command(ADD_CHORE_TYPE.with(ChoreTypeRecord::new(
                            data.get_added_chore_type_name().to_string(),
                            data.get_added_chore_description().to_string(),
                        )))
                    },
                ))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_placeholder("Chore name")
                        .fix_width(PERSON_NAME_FIELD_WIDTH)
                        .lens(MainStateInputData::added_chore_type_name),
                )
                .with_default_spacer()
                .with_child(
                    TextBox::multiline()
                        .with_placeholder("Chore description")
                        // .expand_width()
                        .fix_width(CHORE_DESCRIPTION_WIDTH)
                        .lens(MainStateInputData::added_chore_description),
                )
                .lens(MainStateData::input_data),
        )
        .fix_width(ITEM_LIST_WIDTH)
}
