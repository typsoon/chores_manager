use crate::view::main_ui_elements::day_widget_grid::build_day_widget_grid;
use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::selectors::CHANGE_MONTH;
use druid::widget::{
    Align, Button, Container, Flex, Label
    ,
};
use druid::{Color, Widget, WidgetExt};

const CELL_WIDTH: f64 = 200.0;
const CELL_HEIGHT: f64 = 60.0;
const DAY_WIDGET_WIDTH: f64 = CELL_WIDTH;
const DAY_WIDGET_HEIGHT: f64 = 100.0;
const FONT_SIZE: f64 = 24.0;
const DAY_WIDGET_ITEM_WIDTH: f64 = DAY_WIDGET_WIDTH - 15.0;
const DAY_WIDGET_ITEM_HEIGHT: f64 = 20.0;
const CHORES_DESC_TEXT_SIZE: f64 = 10.0;

// pub fn build_calendar_widget(chores_data: &ChoresData, month_data: MonthData, viewmodel: Box<dyn ViewModel>) -> impl Widget<AppState> {
pub fn build_calendar_widget() -> impl Widget<DatabaseData> {
    let make_label = |label_name| {
        let label = Label::new(label_name).with_text_size(FONT_SIZE).center();

        Container::new(Align::centered(label))
            .background(Color::rgb(0.6, 0.8, 0.9)) // Light blue background
            .border(Color::BLACK, 2.0) // Black border around the rectangle
            .fix_size(CELL_WIDTH, CELL_HEIGHT)
    };

    let column = Flex::column()
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
            Flex::row()
                .with_child(make_label("Mon"))
                .with_child(make_label("Tue"))
                .with_child(make_label("Wed"))
                .with_child(make_label("Thu"))
                .with_child(make_label("Fri"))
                .with_child(make_label("Sat"))
                .with_child(make_label("Sun")),
        )
        // .with_child(
        //     CalendarGridController::create_day_widgets_grid(&MonthData::current())
        //         .controller(CalendarGridController),
        // );
        .with_child(build_day_widget_grid());

    // CalendarGridController::create_day_widgets_grid(, &mut column);
    column
}
