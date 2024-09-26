use crate::model::types::{CompletedChoreData, OneTimeChoreRecord};
use crate::view::main_ui_elements::dropdown_button::get_dropdown_button;
use crate::view::view_types::selectors::{ADD_ONE_TIME_CHORE, COMPLETE_CHORE};
use crate::view::view_types::wrappers::{
    ChoreTypeRecordWrapper, ChoresDataKeyVal, FullChoreDataWrapper, FullDayData,
    PersonRecordWrapper,
};
use druid::im::Vector;
use druid::lens::Identity;
use druid::widget::{BackgroundBrush, Button, Either, Flex, Label, List, Painter, Scope, TextBox};
use druid::LensExt;
use druid::{Color, Data, EventCtx, Lens, Target, Widget, WidgetExt};

const CHORES_DESC_TEXT_SIZE: f64 = 20.0;

pub fn get_chore_box_painter() -> Painter<FullChoreDataWrapper> {
    Painter::new(|ctx: &mut _, data: &FullChoreDataWrapper, env: &_| {
        let mut brush = BackgroundBrush::Color(if data.was_completed() {
            Color::GREEN
        } else {
            Color::RED
        });
        brush.paint(ctx, data, env)
    })
}

#[derive(Clone, Data, Lens)]
struct ChoreItemPrivateData {
    full_chore_data_record: FullChoreDataWrapper,
    is_selected: bool,
    completed_message: String,
}

impl ChoreItemPrivateData {
    pub fn new(full_chore_data_record: FullChoreDataWrapper) -> Self {
        Self {
            full_chore_data_record,
            is_selected: false,
            completed_message: Default::default(),
        }
    }
}

fn build_list_of_chores() -> impl Widget<FullDayData> {
    List::new(|| {
        Scope::from_lens(
            ChoreItemPrivateData::new,
            ChoreItemPrivateData::full_chore_data_record,
            Flex::column()
                .with_child(
                    Label::new(|item: &FullChoreDataWrapper, _env: &_| {
                        format!("{}\t{}\n", item.chore_name(), item.person_name())
                    })
                    .with_text_size(CHORES_DESC_TEXT_SIZE)
                    .lens(ChoreItemPrivateData::full_chore_data_record)
                    .padding((0., 5., 0., 0.))
                    .on_click(
                        |_: &mut _, data: &mut ChoreItemPrivateData, _: &_| data.is_selected = true,
                    ),
                )
                .with_child(Either::new(
                    |data: &ChoreItemPrivateData, _| data.is_selected,
                    Flex::row()
                        .with_child(
                            TextBox::multiline().lens(ChoreItemPrivateData::completed_message),
                        )
                        .with_default_spacer()
                        .with_child(Button::new("Complete").on_click(
                            |ctx: &mut EventCtx, data: &mut ChoreItemPrivateData, _: &_| {
                                ctx.submit_command(
                                    COMPLETE_CHORE
                                        .with(CompletedChoreData::new(
                                            data.full_chore_data_record.chore_name().to_string(),
                                            data.full_chore_data_record.iteration(),
                                            data.completed_message.clone(),
                                        ))
                                        .to(Target::Global),
                                )
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Button::new("X").on_click(
                            |_: &mut _, data: &mut ChoreItemPrivateData, _: &_| {
                                data.is_selected = false
                            },
                        ))
                        .padding((0., 0., 0., 5.)),
                    Flex::column(),
                )),
        )
        .background(get_chore_box_painter())
        .padding((5., 0., 5., 10.))
    })
    .scroll()
    .fix_height(320.)
    .lens(FullDayData::keyval.then(ChoresDataKeyVal::chores))
}

#[derive(Clone, Data, Lens)]
struct AddChoresWidgetPrivateData {
    selected_person: PersonRecordWrapper,
    selected_chore_name: String,
    full_day_data: FullDayData,
}

impl AddChoresWidgetPrivateData {
    pub fn new(full_day_data: FullDayData) -> Self {
        Self {
            selected_person: full_day_data.get_people().first().cloned().unwrap_or_default(),
            selected_chore_name: full_day_data.get_chores().first().cloned().unwrap_or_default().chore_name().to_string(),
            full_day_data,
        }
    }
}

fn build_add_chores_widget() -> impl Widget<FullDayData> {
    Scope::from_lens(
        AddChoresWidgetPrivateData::new,
        AddChoresWidgetPrivateData::full_day_data,
        // TODO: change this bad code below

        // AddChoresWidgetPrivateData::full_day_data.then(FullDayData::people),
        Flex::row()
            .with_child(
                get_dropdown_button().lens(Identity.map(
                    |data: &AddChoresWidgetPrivateData| {
                        (
                            data.selected_person.clone(),
                            data.full_day_data.get_people()
                                        .iter()
                                        .cloned()
                                        .collect::<Vector<PersonRecordWrapper>>()
                        )
                    },
                    |data: &mut AddChoresWidgetPrivateData, new_data: (PersonRecordWrapper, _)| {
                        data.selected_person = new_data.0;
                        // data.full_day_data.get_people() = Arc::new(new_data.1.iter().cloned().collect());
                    },
                    ),
                ),
            )
            .with_default_spacer()
            .with_child(
                get_dropdown_button().lens(
                    Identity.map(
                        |data: &AddChoresWidgetPrivateData| {
                            (
                                data.selected_chore_name.clone(),
                                data.full_day_data.get_chores()
                                            .iter()
                                            .map(|chore_type_wrapper: &ChoreTypeRecordWrapper| {
                                                chore_type_wrapper.chore_name().to_string()
                                            })
                                            .collect::<Vector<String>>()
                            )
                        },
                        |data: &mut AddChoresWidgetPrivateData, new_data: (String, _)| {
                            data.selected_chore_name = new_data.0;
                            // data.full_day_data.get_chores() = Arc::new(new_data.1.iter().cloned().collect());
                        },
                    ),
                ),
            )
            .with_default_spacer()
            .with_child(Button::new("Add one time chore").on_click(|ctx: &mut EventCtx, data: &mut AddChoresWidgetPrivateData, _| {
                ctx.submit_command(
                    ADD_ONE_TIME_CHORE
                        .with(OneTimeChoreRecord::new(
                            data.selected_person.to_string(),
                            data.selected_chore_name.clone(),
                            data.full_day_data.get_day(),
                        ))
                        .to(Target::Global),
                )
            })),
    )
}

pub fn build_sub_window_widget() -> impl Widget<FullDayData> {
    Flex::column()
        .with_child(build_list_of_chores())
        .with_child(build_add_chores_widget())
        .center()
}
