use crate::view::configure_env::{
    CALENDAR_ACTIVE_MONTH_CONTAINER_COLOR, CALENDAR_INACTIVE_MONTH_CONTAINER_COLOR,
};
use crate::view::view_types::wrappers::FullChoreDataWrapper;
use chrono::{Datelike, NaiveDate};
use druid::widget::{BackgroundBrush, Painter};
use druid::{Color, Key};

pub fn get_container_color(day: NaiveDate, active_month: u32) -> Key<Color> {
    if day.month() == active_month {
        CALENDAR_ACTIVE_MONTH_CONTAINER_COLOR
    } else {
        CALENDAR_INACTIVE_MONTH_CONTAINER_COLOR
    }
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
