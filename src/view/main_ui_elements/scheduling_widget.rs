use crate::view::configure_env::{
    CALENDAR_DAY_NUMBER_LABEL_COLOR, CALENDAR_WEEKDAY_LABEL_COLOR,
    SCHEDULING_WIDGET_WEEKDAY_LABEL_FONT, SCHEDULING_WIDGET_WEEKDAY_LABEL_HEIGHT,
    SCHEDULING_WIDGET_WEEKDAY_LABEL_WIDTH,
};
use crate::view::main_ui_elements::generic_calendar_builder::build_calendar;
use crate::view::view_types::utils::MonthData;
use crate::view::view_types::wrappers::NaiveDateWrapper;
use chrono::NaiveDate;
use druid::im::Vector;
use druid::widget::{BackgroundBrush, Flex, Label, Painter, Scope};
use druid::{Color, Data, Lens, Widget, WidgetExt};
use std::ops::Deref;

type ListOfDays = Vector<DayWidgetData>;

struct DayWidgetDataLens;

impl DayWidgetDataLens {
    fn get_days(scheduling_widget_data: &SchedulingWidgetData) -> Vector<DayWidgetData> {
        scheduling_widget_data
            .month_data
            .get_important_days()
            .iter()
            .map(|important_day| {
                DayWidgetData::new(
                    todo!(),
                    important_day == scheduling_widget_data.first_day.deref(),
                    scheduling_widget_data.second_day == Some(*important_day),
                    *important_day,
                )
            })
            .collect()
    }
}

impl Lens<SchedulingWidgetData, Vector<DayWidgetData>> for DayWidgetDataLens {
    fn with<V, F: FnOnce(&Vector<DayWidgetData>) -> V>(
        &self,
        data: &SchedulingWidgetData,
        f: F,
    ) -> V {
        f(&Self::get_days(data))
    }

    fn with_mut<V, F: FnOnce(&mut Vector<DayWidgetData>) -> V>(
        &self,
        data: &mut SchedulingWidgetData,
        f: F,
    ) -> V {
        f(&mut Self::get_days(data))
    }
}

#[derive(Clone, Data, Lens)]
struct SchedulingWidgetData {
    month_data: MonthData,
    first_day: NaiveDateWrapper,
    #[data(eq)]
    second_day: Option<NaiveDate>,
    #[data(eq)]
    last_day: Option<NaiveDate>,
}

impl SchedulingWidgetData {
    pub fn new(first_day: NaiveDateWrapper) -> Self {
        Self {
            month_data: MonthData::get_from_date_like(*first_day.deref()).unwrap(),
            first_day,
            second_day: Default::default(),
            last_day: Default::default(),
        }
    }
}

#[derive(Clone, Data)]
struct DayWidgetData {
    is_selected: bool,
    is_first_day: bool,
    is_second_day: bool,
    #[data(eq)]
    day: NaiveDate,
}

impl DayWidgetData {
    pub fn new(is_selected: bool, is_first_day: bool, is_second_day: bool, day: NaiveDate) -> Self {
        Self {
            is_selected,
            is_first_day,
            is_second_day,
            day,
        }
    }
}

fn weekday_label_builder() -> impl Widget<String> {
    Label::new(|day_name: &String, _: &_| day_name.clone())
        .with_font(SCHEDULING_WIDGET_WEEKDAY_LABEL_FONT)
        // .with_text_color(Color::BLACK)
        .center()
        .background(CALENDAR_WEEKDAY_LABEL_COLOR)
        .border(Color::BLACK, 2.0)
        .rounded(5.0)
        .fix_size(
            SCHEDULING_WIDGET_WEEKDAY_LABEL_WIDTH,
            SCHEDULING_WIDGET_WEEKDAY_LABEL_HEIGHT,
        )
}

fn day_widget_builder() -> impl Widget<DayWidgetData> {
    Flex::column()
        .on_added(|column, _, data: &DayWidgetData, _env| {
            column.add_child(
                Label::new(data.day.to_string())
                    // .with_text_color(Color::BLACK)
                    .with_text_color(CALENDAR_DAY_NUMBER_LABEL_COLOR)
                    .padding((5., 0., 5., 0.)),
            )
        })
        .background(BackgroundBrush::Painter(Painter::new(|ctx, data: &DayWidgetData, env| {
            let color = if data.is_selected {
                Color::BLUE
            } else if data.is_first_day {
                Color::GREEN
            } else if data.is_second_day {
                Color::RED
            } else {
                Color::WHITE
            };
            BackgroundBrush::Color(color).paint(ctx, data, env)
        })))
}

pub fn build_scheduling_widget() -> impl Widget<NaiveDateWrapper> {
    Scope::from_lens(
        SchedulingWidgetData::new,
        SchedulingWidgetData::first_day,
        build_calendar(weekday_label_builder, day_widget_builder).lens(DayWidgetDataLens),
    )
}
