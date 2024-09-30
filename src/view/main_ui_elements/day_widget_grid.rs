use crate::view::configure_env::{
    CALENDAR_ACTIVE_MONTH_CONTAINER_COLOR, CALENDAR_DAY_NUMBER_LABEL_COLOR,
    CALENDAR_INACTIVE_MONTH_CONTAINER_COLOR, CALENDAR_ITEM_HEIGHT, CALENDAR_ITEM_WIDTH,
    CALENDAR_WEEKDAY_LABEL_COLOR, CHORES_DESC_TEXT_SIZE, CHORE_LIST_HEIGHT, DAY_NAME_CELL_HEIGHT,
    DAY_NAME_CELL_WIDTH, DAY_NAME_FONT_SIZE, SUB_WINDOW_WIDGET_WINDOW_SIZE,
};
use crate::view::main_ui_elements::generic_calendar_builder::build_calendar;
use crate::view::main_ui_elements::sub_window_widget;
use crate::view::main_ui_elements::sub_window_widget::build_sub_window_widget;
use crate::view::view_types::wrappers::{
    ChoresDataKeyVal, FullChoreDataWrapper, FullDayData, ImportantDays,
};
use chrono::Datelike;
use druid::im::vector;
use druid::widget::{BackgroundBrush, Controller, Flex, Label, List, Painter};
use druid::{Color, Env, Key, UpdateCtx, Widget, WidgetExt, WindowConfig, WindowLevel};

struct TempController;

// TODO: delete this controller and find a different way
/// Disgusting fix but it works

impl<W: Widget<ImportantDays>> Controller<ImportantDays, W> for TempController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &ImportantDays,
        data: &ImportantDays,
        env: &Env,
    ) {
        child.update(ctx, old_data, &vector![], env);
        child.update(ctx, &vector![], data, env);
        // child.update(ctx, old_data, data, env);
    }
}

fn get_container_color(chores_data_key_val: &ChoresDataKeyVal) -> Key<Color> {
    if chores_data_key_val.get_day().month() == chores_data_key_val.get_month() {
        CALENDAR_ACTIVE_MONTH_CONTAINER_COLOR
    } else {
        CALENDAR_INACTIVE_MONTH_CONTAINER_COLOR
    }
}

fn weekday_label_builder() -> impl Widget<String> {
    Label::new(|day_name: &String, _: &_| day_name.clone())
        .with_text_size(DAY_NAME_FONT_SIZE)
        // .with_text_color(Color::BLACK)
        .center()
        .background(CALENDAR_WEEKDAY_LABEL_COLOR)
        .border(Color::BLACK, 2.0)
        .rounded(5.0)
        .fix_size(DAY_NAME_CELL_WIDTH, DAY_NAME_CELL_HEIGHT)
}

pub fn build_day_widget_grid() -> impl Widget<ImportantDays> {
    build_calendar(weekday_label_builder, get_day_widget).controller(TempController)
}

fn get_chore_list() -> impl Widget<ChoresDataKeyVal> {
    List::new(|| {
        Label::new(|item: &FullChoreDataWrapper, _env: &_| {
            format!("{}\t{}", item.chore_name(), item.person_name())
        })
        // .with_text_color(Color::BLACK)
        .with_text_size(CHORES_DESC_TEXT_SIZE)
        .background(sub_window_widget::get_chore_box_painter())
        .padding((0., 5., 0., 0.))
    })
    .scroll()
    .fix_height(CHORE_LIST_HEIGHT)
    .lens(ChoresDataKeyVal::chores)
    // .debug_paint_layout()
}

fn get_day_widget() -> impl Widget<FullDayData> {
    Flex::column()
        .on_added(|column, _, keyval: &ChoresDataKeyVal, &_| {
            column.add_child(
                Label::new(keyval.get_day().day().to_string())
                    // .with_text_color(Color::BLACK)
                    .with_text_color(CALENDAR_DAY_NUMBER_LABEL_COLOR)
                    .padding((5., 0., 5., 0.)),
            );

            column.add_child(get_chore_list());
        })
        .background(Painter::new(
            move |ctx: &mut _, keyval: &ChoresDataKeyVal, env: &_| {
                BackgroundBrush::ColorKey(get_container_color(keyval)).paint(ctx, keyval, env);
            },
        ))
        .lens(FullDayData::keyval)
        .border(Color::BLACK, 2.0)
        .rounded(5.)
        .fix_size(CALENDAR_ITEM_WIDTH, CALENDAR_ITEM_HEIGHT)
        .on_click(|ctx, full_day_data: &mut FullDayData, env| {
            ctx.new_sub_window(
                WindowConfig::default()
                    .show_titlebar(true)
                    .window_size(env.get(SUB_WINDOW_WIDGET_WINDOW_SIZE))
                    .set_level(WindowLevel::AppWindow),
                build_sub_window_widget(),
                full_day_data.clone(),
                env.clone(),
            );
        })
}
