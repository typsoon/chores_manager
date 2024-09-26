use crate::view::configure_env::{DAY_NAME_CELL_HEIGHT, DAY_NAME_CELL_WIDTH, DAY_NAME_FONT_SIZE};
use crate::view::main_ui_elements::day_widget_grid::build_day_widget_grid;
use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::selectors::CHANGE_MONTH;
use druid::im::{vector, Vector};
use druid::lens::Constant;
use druid::widget::{Button, Flex, Label, List};
use druid::{Color, Widget, WidgetExt};

pub fn build_calendar_widget() -> impl Widget<DatabaseData> {
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
        .with_child(
            List::new(|| {
                Label::new(|day_name: &String, _: &_| day_name.clone())
                    .with_text_size(DAY_NAME_FONT_SIZE)
                    // .with_text_color(Color::BLACK)
                    .center()
                    .background(Color::rgb(0.6, 0.8, 0.9))
                    .border(Color::BLACK, 2.0)
                    .rounded(5.0)
                    .fix_size(DAY_NAME_CELL_WIDTH, DAY_NAME_CELL_HEIGHT)
            })
            .horizontal()
            .lens(Constant(
                vector!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vector<String>>(),
            )),
        )
        .with_child(build_day_widget_grid())
}
