use crate::view::main_ui_elements::day_widget_grid::build_day_widget_grid;
use crate::view::view_types::app_state::{DatabaseData, ImportantWeeksNewLens};
use crate::view::view_types::selectors::CHANGE_MONTH;
use druid::widget::{Button, Flex, Label};
use druid::{Widget, WidgetExt};

pub fn build_chores_overview() -> impl Widget<DatabaseData> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Button::new("Prev month").on_click(
                    |ctx, data: &mut DatabaseData, _| {
                        ctx.submit_command(
                            CHANGE_MONTH.with(data.get_month_data().get_prev_month()),
                        )
                    },
                ))
                .with_default_spacer()
                .with_child(Label::new(|data: &DatabaseData, _: &_| {
                    data.get_month_data()
                        .first_day()
                        .format("%B %Y")
                        .to_string()
                }))
                .with_default_spacer()
                .with_child(Button::new("Next month").on_click(
                    |ctx, data: &mut DatabaseData, _| {
                        ctx.submit_command(
                            CHANGE_MONTH.with(data.get_month_data().get_next_month()),
                        )
                    },
                )),
        )
        .with_default_spacer()
        // .with_child(build_day_widget_grid().lens(ImportantWeeksLens))
        .with_child(build_day_widget_grid().lens(ImportantWeeksNewLens))
}
