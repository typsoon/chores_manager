use crate::view::main_ui_elements::day_widget_grid::build_day_widget_grid;
use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::selectors::CHANGE_MONTH;
use druid::im::{vector, Vector};
use druid::lens::Constant;
use druid::widget::{Button, Flex, Label, List};
use druid::{Color, Widget, WidgetExt};

const CELL_WIDTH: f64 = 200.0;
const CELL_HEIGHT: f64 = 60.0;
const DAY_WIDGET_WIDTH: f64 = CELL_WIDTH;
const DAY_WIDGET_HEIGHT: f64 = 100.0;
const FONT_SIZE: f64 = 24.0;
const DAY_WIDGET_ITEM_WIDTH: f64 = DAY_WIDGET_WIDTH - 15.0;
const DAY_WIDGET_ITEM_HEIGHT: f64 = 20.0;
const CHORES_DESC_TEXT_SIZE: f64 = 10.0;

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
                    .with_text_size(FONT_SIZE)
                    .center()
                    .background(Color::rgb(0.6, 0.8, 0.9))
                    .border(Color::BLACK, 2.0)
                    .rounded(5.0)
                    .fix_size(CELL_WIDTH, CELL_HEIGHT)
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
