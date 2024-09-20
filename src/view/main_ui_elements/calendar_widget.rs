use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::utils::DateUtils;
use crate::view::view_types::utils::MonthData;
use crate::view::view_types::wrappers::FullChoreDataWrapper;
use chrono::{Datelike, NaiveDate, Weekday};
use druid::im::Vector;
use druid::widget::{Align, Container, Controller, Flex, Label, List, Padding, Scroll};
use druid::{Color, Env, Lens, UpdateCtx, Widget, WidgetExt};
use std::mem;

const CELL_WIDTH: f64 = 200.0;
const CELL_HEIGHT: f64 = 60.0;
const DAY_WIDGET_WIDTH: f64 = CELL_WIDTH;
const DAY_WIDGET_HEIGHT: f64 = 100.0;
const FONT_SIZE: f64 = 24.0;
const DAY_WIDGET_ITEM_WIDTH: f64 = DAY_WIDGET_WIDTH - 15.0;
const DAY_WIDGET_ITEM_HEIGHT: f64 = 20.0;
const CHORES_DESC_TEXT_SIZE: f64 = 10.0;

pub struct CalendarWidgetController;

impl CalendarWidgetController {
    fn create_day_widgets(month_data: &MonthData, widget: &mut Flex<DatabaseData>) {
        let current_month_tiles_color = Color::rgb(0.7, 0.9, 1.0);
        let other_tiles_color = Color::rgb(0.8, 0.8, 0.8);
        let get_right_color = |day: &NaiveDate| {
            if day.month() == month_data.first_day.month() {
                current_month_tiles_color
            } else {
                other_tiles_color
            }
        };

        let important_days = DateUtils::get_important_days(month_data.first_day);
        let mut row = Flex::row();

        important_days.iter().for_each(|day| {
            let container = Container::new(build_day_widget_list(*day))
                .background(get_right_color(day)) // Darker blue background
                .border(Color::BLACK, 2.0) // Black border around the rectangle
                .fix_size(DAY_WIDGET_WIDTH, DAY_WIDGET_HEIGHT);

            row.add_child(container);
            if day.weekday() == Weekday::Sun {
                widget.add_child(mem::replace(&mut row, Flex::row()));
            }
        });
    }
}

impl Controller<DatabaseData, Flex<DatabaseData>> for CalendarWidgetController {
    fn update(
        &mut self,
        child: &mut Flex<DatabaseData>,
        ctx: &mut UpdateCtx,
        old_data: &DatabaseData,
        data: &DatabaseData,
        env: &Env,
    ) {
        Self::create_day_widgets(data.get_month_data(), child);
        child.update(ctx, old_data, data, env)
    }
}

struct DayWidgetLens {
    date: NaiveDate,
}

impl DayWidgetLens {
    pub fn new(date: NaiveDate) -> Self {
        Self { date }
    }

    fn get_vector(&self, data: &DatabaseData) -> Vector<FullChoreDataWrapper> {
        data.get_chores_data()
            .get(&self.date)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| FullChoreDataWrapper::new(x.clone()))
            .collect()
    }
}

impl Lens<DatabaseData, Vector<FullChoreDataWrapper>> for DayWidgetLens {
    fn with<V, F: FnOnce(&Vector<FullChoreDataWrapper>) -> V>(
        &self,
        data: &DatabaseData,
        f: F,
    ) -> V {
        f(&self.get_vector(data))
    }

    fn with_mut<V, F: FnOnce(&mut Vector<FullChoreDataWrapper>) -> V>(
        &self,
        data: &mut DatabaseData,
        f: F,
    ) -> V {
        f(&mut self.get_vector(data))
    }
}

fn build_day_widget_list(day: NaiveDate) -> impl Widget<DatabaseData> {
    let column = Flex::column().with_child(Padding::new(
        (0.0, 10.0, 0.0, 0.0),
        Label::new(day.day().to_string()),
    ));

    let list = List::new(|| {
        let label = Label::new(|item: &FullChoreDataWrapper, _env: &_| {
            format!("{}\t{}", item.chore_name(), item.person_name())
        })
        .with_text_size(CHORES_DESC_TEXT_SIZE);

        Container::new(label)
            .background(Color::BLACK)
            .border(Color::BLACK, 2.0)
            .fix_size(DAY_WIDGET_ITEM_WIDTH, DAY_WIDGET_ITEM_HEIGHT)
    });

    column.with_child(
        Padding::new((5.0, 5.0, 5.0, 0.0), Scroll::new(list)).lens(DayWidgetLens::new(day)),
    )
}

// pub fn build_calendar_widget(chores_data: &ChoresData, month_data: MonthData, viewmodel: Box<dyn ViewModel>) -> impl Widget<AppState> {
pub fn build_calendar_widget() -> impl Widget<DatabaseData> {
    let make_label = |label_name| {
        let label = Label::new(label_name).with_text_size(FONT_SIZE).center();

        Container::new(Align::centered(label))
            .background(Color::rgb(0.6, 0.8, 0.9)) // Light blue background
            .border(Color::BLACK, 2.0) // Black border around the rectangle
            .fix_size(CELL_WIDTH, CELL_HEIGHT)
    };

    let mut column = Flex::column().with_child(
        Flex::row()
            .with_child(make_label("Mon"))
            .with_child(make_label("Tue"))
            .with_child(make_label("Wed"))
            .with_child(make_label("Thu"))
            .with_child(make_label("Fri"))
            .with_child(make_label("Sat"))
            .with_child(make_label("Sun")),
    );

    CalendarWidgetController::create_day_widgets(&MonthData::current(), &mut column);
    column.controller(CalendarWidgetController)
}
