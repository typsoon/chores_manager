use crate::view::configure_env::{
    CALENDAR_DAY_NUMBER_LABEL_COLOR, CALENDAR_WEEKDAY_LABEL_COLOR,
    SCHEDULING_WIDGET_DAY_WIDGET_HEIGHT, SCHEDULING_WIDGET_WEEKDAY_LABEL_FONT,
    SCHEDULING_WIDGET_WEEKDAY_LABEL_HEIGHT, SCHEDULING_WIDGET_WEEKDAY_LABEL_WIDTH,
};
use crate::view::main_ui_elements::generic_calendar_builder::build_calendar;
use crate::view::utils::ui_utils::change_month_widget::get_change_month_widget;
use crate::view::utils::ui_utils::dropdown_button::get_dropdown_button;
use crate::view::utils::ui_utils::painters::get_container_color;
use crate::view::view_types::selectors::CHANGE_MONTH;
use crate::view::view_types::wrappers::NaiveDateWrapper;
use chrono::{Datelike, NaiveDate};
use druid::im::vector;
use druid::lens::Identity;
use druid::widget::{BackgroundBrush, Button, Controller, Flex, Label, Painter, Scope};
use druid::Target::Global;
use druid::{Color, Env, Event, EventCtx, LensExt, Selector, Widget, WidgetExt};
use scheduling_widget_data::{
    DaySelectionState, DayWidgetData, DayWidgetDataLens, SchedulingWidgetData,
};

pub mod scheduling_widget_data {
    pub use crate::view::utils::date_utils::MonthData;
    use crate::view::utils::ui_utils::change_month_widget::ReadableMonthData;
    pub use crate::view::view_types::wrappers::NaiveDateWrapper;
    pub use chrono::{Datelike, Days, NaiveDate};
    use druid::im::{vector, Vector};
    use druid::{Data, Lens};
    use std::fmt::{Display, Formatter};
    use std::ops::Deref;

    type ListOfDays = Vector<DayWidgetData>;

    pub struct DayWidgetDataLens;

    impl DayWidgetDataLens {
        fn get_days(scheduling_widget_data: &SchedulingWidgetData) -> ListOfDays {
            scheduling_widget_data
                .month_data
                .get_important_days()
                .iter()
                .map(|important_day| {
                    DayWidgetData::new(
                        scheduling_widget_data.is_active(*important_day),
                        // false,
                        important_day == scheduling_widget_data.first_day.deref(),
                        scheduling_widget_data.second_day == Some(*important_day),
                        *important_day,
                        scheduling_widget_data.month_data.first_day().month(),
                    )
                })
                .collect()
        }
    }

    impl Lens<SchedulingWidgetData, ListOfDays> for DayWidgetDataLens {
        fn with<V, F: FnOnce(&ListOfDays) -> V>(&self, data: &SchedulingWidgetData, f: F) -> V {
            f(&Self::get_days(data))
        }

        fn with_mut<V, F: FnOnce(&mut ListOfDays) -> V>(
            &self,
            data: &mut SchedulingWidgetData,
            f: F,
        ) -> V {
            f(&mut Self::get_days(data))
        }
    }

    #[derive(Clone, Data, Default, PartialEq, Copy)]
    pub enum DaySelectionState {
        #[default]
        SecondDay,
        LastDay,
    }

    impl Display for DaySelectionState {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            (match self {
                DaySelectionState::SecondDay => "Select second day",
                DaySelectionState::LastDay => "Select last day",
            })
            .fmt(f)
        }
    }

    #[derive(Clone, Data, Lens)]
    pub struct SchedulingWidgetData {
        day_selection_state: DaySelectionState,
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
                day_selection_state: Default::default(),
                month_data: MonthData::get_from_date_like(*first_day.deref()).unwrap(),
                first_day,
                // second_day: Default::default(),
                second_day: Some(first_day.deref().checked_add_days(Days::new(3)).unwrap()),
                last_day: Default::default(),
            }
        }

        fn is_active(&self, day: NaiveDate) -> bool {
            if let Some(unwrapped_second_day) = self.second_day {
                (day - *self.first_day).num_days()
                    % (unwrapped_second_day - *self.first_day).num_days()
                    == 0
                    && day > *self.first_day
                    && if let Some(unwrapped_last_day) = self.last_day {
                        day <= unwrapped_last_day
                    } else {
                        true
                    }
            } else {
                false
            }
        }

        pub fn set_month_data(&mut self, month_data: MonthData) {
            self.month_data = month_data;
        }

        pub fn set_second_day(&mut self, second_day: NaiveDate) {
            if second_day > *self.first_day {
                self.second_day = Some(second_day);
            }
        }

        pub fn set_last_day(&mut self, last_day: NaiveDate) {
            if last_day > *self.first_day {
                self.last_day = Some(last_day);
            }
        }

        pub fn get_day_selection_state(&self) -> DaySelectionState {
            self.day_selection_state
        }

        pub fn set_day_selection_state(&mut self, day_selection_state: DaySelectionState) {
            self.day_selection_state = day_selection_state;
        }

        pub fn get_second_day(&self) -> Option<NaiveDate> {
            self.second_day
        }

        pub fn get_last_day(&self) -> Option<NaiveDate> {
            self.last_day
        }
    }

    impl ReadableMonthData for SchedulingWidgetData {
        fn get_month_data(&self) -> MonthData {
            self.month_data
        }
    }

    pub struct DaySelectionStateLens;

    impl DaySelectionStateLens {
        fn get_tuple(
            data: &SchedulingWidgetData,
        ) -> (DaySelectionState, Vector<DaySelectionState>) {
            (
                data.day_selection_state,
                vector![DaySelectionState::SecondDay, DaySelectionState::LastDay],
            )
        }
    }

    impl Lens<SchedulingWidgetData, (DaySelectionState, Vector<DaySelectionState>)>
        for DaySelectionStateLens
    {
        fn with<V, F: FnOnce(&(DaySelectionState, Vector<DaySelectionState>)) -> V>(
            &self,
            data: &SchedulingWidgetData,
            f: F,
        ) -> V {
            f(&Self::get_tuple(data))
        }

        fn with_mut<V, F: FnOnce(&mut (DaySelectionState, Vector<DaySelectionState>)) -> V>(
            &self,
            data: &mut SchedulingWidgetData,
            f: F,
        ) -> V {
            f(&mut Self::get_tuple(data))
        }
    }

    #[derive(Clone, Data)]
    pub struct DayWidgetData {
        is_selected: bool,
        is_first_day: bool,
        is_second_day: bool,
        #[data(eq)]
        day: NaiveDate,
        active_month: u32,
    }

    impl DayWidgetData {
        pub fn new(
            is_selected: bool,
            is_first_day: bool,
            is_second_day: bool,
            day: NaiveDate,
            active_month: u32,
        ) -> Self {
            Self {
                is_selected,
                is_first_day,
                is_second_day,
                day,
                active_month,
            }
        }

        pub fn is_selected(&self) -> bool {
            self.is_selected
        }

        pub fn is_first_day(&self) -> bool {
            self.is_first_day
        }

        pub fn is_second_day(&self) -> bool {
            self.is_second_day
        }

        pub fn day(&self) -> NaiveDate {
            self.day
        }

        pub fn get_active_month(&self) -> u32 {
            self.active_month
        }
    }
}

#[derive(Clone, Copy)]
pub struct SecondDayLastDay {
    pub second_day: NaiveDate,
    pub last_day: NaiveDate,
}

pub const DATA_SUBMITTED: Selector<SecondDayLastDay> = Selector::new("data_submitted");

const SUBMIT_CLICKED: Selector = Selector::new("submit_clicked");

const DAY_SELECTED: Selector<NaiveDate> = Selector::new("day_selected");

struct SchedulingWidgetController;

impl<T: Widget<SchedulingWidgetData>> Controller<SchedulingWidgetData, T>
    for SchedulingWidgetController
{
    fn event(
        &mut self,
        child: &mut T,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut SchedulingWidgetData,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
            if let Some(month_data) = cmd.get(CHANGE_MONTH) {
                data.set_month_data(*month_data);
            }

            if let Some(day) = cmd.get(DAY_SELECTED) {
                match data.get_day_selection_state() {
                    DaySelectionState::SecondDay => data.set_second_day(*day),
                    DaySelectionState::LastDay => data.set_last_day(*day),
                }
            }

            if let Some(_) = cmd.get(SUBMIT_CLICKED) {
                if let (Some(unwrapped_second_day), Some(unwrapped_last_day)) =
                    (data.get_second_day(), data.get_last_day())
                {
                    ctx.submit_command(
                        DATA_SUBMITTED
                            .with(SecondDayLastDay {
                                second_day: unwrapped_second_day,
                                last_day: unwrapped_last_day,
                            })
                            .to(Global),
                    );
                }
            }
        }

        child.event(ctx, event, data, env);
    }
}

fn weekday_label_builder() -> impl Widget<String> {
    Label::dynamic(|day_name: &String, _: &_| day_name.clone())
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
    Flex::column().with_child(
        Label::dynamic(|data: &DayWidgetData, _: &Env| data.day().day().to_string())
            .with_text_color(CALENDAR_DAY_NUMBER_LABEL_COLOR)
            .padding((5., 0., 5., 0.))
            .background(BackgroundBrush::Painter(Painter::new(
                |ctx, data: &DayWidgetData, env| {
                    let mut brush = if data.is_first_day() {
                        BackgroundBrush::Color(Color::GREEN)
                    } else if data.is_second_day() {
                        BackgroundBrush::Color(Color::RED)
                    } else if data.is_selected() {
                        BackgroundBrush::Color(Color::BLUE)
                    } else {
                        BackgroundBrush::ColorKey(get_container_color(
                            data.day(),
                            data.get_active_month(),
                        ))
                    };
                    brush.paint(ctx, data, env)
                },
            )))
            .rounded(5.)
            .border(Color::BLACK, 2.0)
            .fix_size(
                SCHEDULING_WIDGET_WEEKDAY_LABEL_WIDTH,
                SCHEDULING_WIDGET_DAY_WIDGET_HEIGHT,
            )
            .on_click(|ctx, data, _| ctx.submit_command(DAY_SELECTED.with(data.day()))),
    )
}

pub fn build_scheduling_widget() -> impl Widget<NaiveDateWrapper> {
    Scope::from_lens(
        SchedulingWidgetData::new,
        SchedulingWidgetData::first_day,
        Flex::column()
            .with_child(get_change_month_widget())
            .with_default_spacer()
            .with_child(
                build_calendar(weekday_label_builder, day_widget_builder)
                    .lens(DayWidgetDataLens)
                    .controller(SchedulingWidgetController),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(get_dropdown_button().lens(Identity.map(
                        |data: &SchedulingWidgetData| {
                            (
                                data.get_day_selection_state(),
                                vector![DaySelectionState::SecondDay, DaySelectionState::LastDay],
                            )
                        },
                        |old_data: &mut SchedulingWidgetData, put| {
                            old_data.set_day_selection_state(put.0)
                        },
                    )))
                    .with_default_spacer()
                    .with_child(
                        Button::new("Submit")
                            .on_click(|ctx, _, _| ctx.submit_command(SUBMIT_CLICKED)),
                    ),
            ),
    )
}
