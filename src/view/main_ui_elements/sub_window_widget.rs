use crate::model::types::CompletedChoreData;
use crate::view::view_types::selectors::COMPLETE_CHORE;
use crate::view::view_types::wrappers::{ChoresDataKeyVal, FullChoreDataWrapper};
use druid::widget::{BackgroundBrush, Button, Either, Flex, Label, List, Painter, Scope, TextBox};
use druid::{Color, Data, EventCtx, Lens, Target, Widget, WidgetExt};

const CHORES_DESC_TEXT_SIZE: f64 = 20.0;

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

pub fn build_sub_window_widget() -> impl Widget<ChoresDataKeyVal> {
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
    .fix_height(500.)
    .lens(ChoresDataKeyVal::chores)
    .center()
}

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
