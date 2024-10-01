use crate::view::utils::date_utils::MonthData;
use crate::view::view_types::selectors::CHANGE_MONTH;
use druid::widget::{Button, Flex, Label};
use druid::{Data, Widget};

pub trait ReadableMonthData {
    fn get_month_data(&self) -> MonthData;
}

pub fn get_change_month_widget<T: Data + ReadableMonthData>() -> impl Widget<T> {
    Flex::row()
        .with_child(Button::new("Prev month").on_click(|ctx, data: &mut T, _| {
            ctx.submit_command(CHANGE_MONTH.with(data.get_month_data().get_prev_month()))
        }))
        .with_default_spacer()
        .with_child(Label::dynamic(|data: &T, _: &_| {
            data.get_month_data()
                .first_day()
                .format("%B %Y")
                .to_string()
        }))
        .with_default_spacer()
        .with_child(Button::new("Next month").on_click(|ctx, data: &mut T, _| {
            ctx.submit_command(CHANGE_MONTH.with(data.get_month_data().get_next_month()))
        }))
}
