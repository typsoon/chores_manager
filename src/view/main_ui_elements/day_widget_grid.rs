use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::wrappers::{ChoresDataKeyVal, FullChoreDataWrapper, ImportantWeeks};
use chrono::Datelike;
use druid::widget::{BackgroundBrush, Container, Controller, Flex, Label, List, Padding, Painter, Scroll};
use druid::{Color, Data, Env, Lens, UpdateCtx, Widget, WidgetExt};
use druid::im::{vector, Vector};

const CELL_WIDTH: f64 = 200.0;
const CELL_HEIGHT: f64 = 60.0;
const DAY_WIDGET_WIDTH: f64 = CELL_WIDTH;
const DAY_WIDGET_HEIGHT: f64 = 100.0;
const DAY_WIDGET_ITEM_WIDTH: f64 = 85.0;
const DAY_WIDGET_ITEM_HEIGHT: f64 = 20.0;
const CHORES_DESC_TEXT_SIZE: f64 = 10.0;

struct TempController;

// TODO: delete this controller and find a different way
/// Disgusting fix but it works
impl<T: Data> Controller<Vector<T>, List<T>> for TempController {
    fn update(&mut self, child: &mut List<T>, ctx: &mut UpdateCtx, old_data: &Vector<T>, data: &Vector<T>, env: &Env) {
        child.update(ctx, old_data, &vector![], env);
        child.update(ctx, &vector![], data, env);
        // child.update(ctx, old_data, data, env);
    }
}

fn get_container_color(chores_data_key_val: &ChoresDataKeyVal) -> Color {
    if chores_data_key_val.get_day().month() == chores_data_key_val.get_month() {
        Color::rgb(0.7, 0.9, 1.0)
    } else {
        Color::rgb(0.8, 0.8, 0.8)
    }
}

struct ImportantWeeksLens;

impl Lens<DatabaseData, ImportantWeeks> for ImportantWeeksLens {
    fn with<V, F: FnOnce(&ImportantWeeks) -> V>(&self, data: &DatabaseData, f: F) -> V {
        f(&data.get_important_weeks())
    }

    fn with_mut<V, F: FnOnce(&mut ImportantWeeks) -> V>(&self, data: &mut DatabaseData, f: F) -> V {
        f(&mut data.get_important_weeks())
    }
}

/// Each value of [Vector]<[ChoresData]> represents one week
pub fn build_day_widget_grid() -> impl Widget<DatabaseData> {
    Container::new(List::new(|| {
        List::new(|| {
            let day_widget_box_painter =
                Painter::new(move |ctx: &mut _, keyval: &ChoresDataKeyVal, env: &_| {
                    BackgroundBrush::Color(get_container_color(keyval)).paint(ctx, keyval, env);
                });

            Container::new(
                Flex::column().on_added(|column, _, keyval: &ChoresDataKeyVal, &_| {
                    column.add_child(Padding::new(
                        (0.0, 10.0, 0.0, 0.0),
                        Label::new(keyval.get_day().day().to_string()),
                    ));

                    column.add_child(
                        get_chore_list()
                    );
                }),
            )
            .background(day_widget_box_painter)
            .border(Color::BLACK, 2.0)
            .fix_size(DAY_WIDGET_WIDTH, DAY_WIDGET_HEIGHT)
        })
        .horizontal()
        .controller(TempController)
    }))
    .lens(ImportantWeeksLens)
}

fn get_chore_list() -> impl Widget<ChoresDataKeyVal> {
    Padding::new(
        (5.0, 5.0, 5.0, 0.0),
        Scroll::new(List::new(|| {
            let label = Label::new(|item: &FullChoreDataWrapper, _env: &_| {
                format!("{}\t{}", item.chore_name(), item.person_name())
            })
            .with_text_size(CHORES_DESC_TEXT_SIZE);

            let chore_box_painter =
                Painter::new(|ctx: &mut _, data: &FullChoreDataWrapper, env: &_| {
                    let mut brush = BackgroundBrush::Color(if data.was_completed() {
                        Color::GREEN
                    } else {
                        Color::RED
                    });
                    brush.paint(ctx, data, env)
                });

            Container::new(label)
                .background(chore_box_painter)
                .padding((0., 5., 0., 0.))
                .fix_size(DAY_WIDGET_ITEM_WIDTH, DAY_WIDGET_ITEM_HEIGHT)
        })),
    )
    .lens(ChoresDataKeyVal::chores)
}
