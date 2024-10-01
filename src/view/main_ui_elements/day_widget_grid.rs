use crate::view::configure_env::{
    CALENDAR_DAY_NUMBER_LABEL_COLOR, CALENDAR_ITEM_HEIGHT, CALENDAR_ITEM_WIDTH,
    CALENDAR_WEEKDAY_LABEL_COLOR, CHORES_DESC_TEXT_SIZE, CHORE_LIST_HEIGHT, DAY_NAME_CELL_HEIGHT,
    DAY_NAME_CELL_WIDTH, DAY_NAME_FONT_SIZE,
};
use crate::view::main_ui_elements::generic_calendar_builder::build_calendar;
use crate::view::main_ui_elements::sub_window_widget::build_sub_window_widget;
use crate::view::utils::ui_utils::painters;
use crate::view::view_types::wrappers::{
    ChoresDataKeyVal, FullChoreDataWrapper, FullDayData, ImportantDays,
};
use chrono::Datelike;
use druid::widget::{BackgroundBrush, Flex, Label, List, Painter};
use druid::{Color, Widget, WidgetExt, WindowConfig, WindowLevel, WindowSizePolicy};
use std::ops::Deref;

fn weekday_label_builder() -> impl Widget<String> {
    Label::dynamic(|day_name: &String, _: &_| day_name.clone())
        .with_text_size(DAY_NAME_FONT_SIZE)
        // .with_text_color(Color::BLACK)
        .center()
        .background(CALENDAR_WEEKDAY_LABEL_COLOR)
        .border(Color::BLACK, 2.0)
        .rounded(5.0)
        .fix_size(DAY_NAME_CELL_WIDTH, DAY_NAME_CELL_HEIGHT)
}

pub fn build_day_widget_grid() -> impl Widget<ImportantDays> {
    build_calendar(weekday_label_builder, get_day_widget)
}

fn get_chore_list() -> impl Widget<ChoresDataKeyVal> {
    List::new(|| {
        Label::dynamic(|item: &FullChoreDataWrapper, _env: &_| {
            format!("{}\t{}", item.chore_name(), item.person_name())
        })
        // .with_text_color(Color::BLACK)
        .with_text_size(CHORES_DESC_TEXT_SIZE)
        .background(painters::get_chore_box_painter())
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
                BackgroundBrush::ColorKey(painters::get_container_color(
                    *keyval.get_day().deref(),
                    keyval.get_month(),
                ))
                .paint(ctx, keyval, env);
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
                    .window_size_policy(WindowSizePolicy::Content)
                    // .window_size(env.get(SUB_WINDOW_WIDGET_WINDOW_SIZE))
                    .set_level(WindowLevel::AppWindow),
                build_sub_window_widget(),
                full_day_data.clone(),
                env.clone(),
            );
        })
}
