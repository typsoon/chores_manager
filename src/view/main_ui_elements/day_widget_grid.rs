use crate::view::main_ui_elements::sub_window_widget::build_sub_window_widget;
use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::wrappers::{ChoresDataKeyVal, FullChoreDataWrapper, ImportantWeeks};
use chrono::Datelike;
use druid::im::{vector, Vector};
use druid::widget::{BackgroundBrush, Controller, Flex, Label, List, Painter};
use druid::{Color, Data, Env, Lens, Size, UpdateCtx, Widget, WidgetExt, WindowConfig, WindowLevel};

const CELL_WIDTH: f64 = 200.0;
const CELL_HEIGHT: f64 = 60.0;
const DAY_WIDGET_WIDTH: f64 = CELL_WIDTH;
const DAY_WIDGET_HEIGHT: f64 = 100.0;
const DAY_WIDGET_ITEM_WIDTH: f64 = 85.0;
const DAY_WIDGET_ITEM_HEIGHT: f64 = 20.0;
const CHORES_DESC_TEXT_SIZE: f64 = 10.0;
const CHORE_LIST_HEIGHT: f64 = 72.5;

struct TempController;

// TODO: delete this controller and find a different way
/// Disgusting fix but it works
impl<T: Data> Controller<Vector<T>, List<T>> for TempController {
    fn update(
        &mut self,
        child: &mut List<T>,
        ctx: &mut UpdateCtx,
        old_data: &Vector<T>,
        data: &Vector<T>,
        env: &Env,
    ) {
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

pub fn build_day_widget_grid() -> impl Widget<DatabaseData> {
    List::new(|| {
        List::new(|| {
            let day_widget_box_painter =
                Painter::new(move |ctx: &mut _, keyval: &ChoresDataKeyVal, env: &_| {
                    BackgroundBrush::Color(get_container_color(keyval)).paint(ctx, keyval, env);
                });

            Flex::column()
                .on_added(|column, _, keyval: &ChoresDataKeyVal, &_| {
                    column.add_child(
                        Label::new(keyval.get_day().day().to_string()).padding((5., 0., 5., 0.)),
                    );

                    column.add_child(get_chore_list());
                })
                .background(day_widget_box_painter)
                .border(Color::BLACK, 2.0)
                .rounded(5.)
                .fix_size(DAY_WIDGET_WIDTH, DAY_WIDGET_HEIGHT)
                .on_click(|ctx, keyval: &mut ChoresDataKeyVal, env| {
                    ctx.new_sub_window(
                        WindowConfig::default()
                            .show_titlebar(true)
                            .window_size(Size::new(500., 500.))
                            .set_level(WindowLevel::AppWindow),
                        build_sub_window_widget(),
                        keyval.clone(),
                        env.clone(),
                    );
                })
            // .debug_paint_layout()
        })
        .horizontal()
        .controller(TempController)
    })
    .lens(ImportantWeeksLens)
}

fn get_chore_list() -> impl Widget<ChoresDataKeyVal> {
    List::new(|| {
        let chore_box_painter =
            Painter::new(|ctx: &mut _, data: &FullChoreDataWrapper, env: &_| {
                let mut brush = BackgroundBrush::Color(if data.was_completed() {
                    Color::GREEN
                } else {
                    Color::RED
                });
                brush.paint(ctx, data, env)
            });

        Label::new(|item: &FullChoreDataWrapper, _env: &_| {
            format!("{}\t{}", item.chore_name(), item.person_name())
        })
        .with_text_size(CHORES_DESC_TEXT_SIZE)
        .background(chore_box_painter)
        .padding((0., 5., 0., 0.))
    })
    .scroll()
    .fix_height(CHORE_LIST_HEIGHT)
    .lens(ChoresDataKeyVal::chores)
    // .debug_paint_layout()
}
