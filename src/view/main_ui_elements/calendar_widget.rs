use crate::model::traits::ReadOnlyDatabaseService;
use crate::model::types::FullChoreDataRecord;
use crate::view::view_types::app_state::DatabaseData;
use crate::view::view_types::utils::DateUtils;
use crate::view::view_types::utils::MonthData;
use chrono::{Datelike, NaiveDate, Weekday};
use delegate::delegate;
use druid::widget::{Align, Container, Flex, Label, Padding};
use druid::{BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt};
use std::mem;
use std::rc::Rc;

const CELL_WIDTH: f64 = 200.0;
const CELL_HEIGHT: f64 = 60.0;
const DAY_WIDGET_WIDTH: f64 = CELL_WIDTH;
const DAY_WIDGET_HEIGHT: f64 = 100.0;
const FONT_SIZE: f64 = 24.0;
const DAY_WIDGET_ITEM_WIDTH: f64 = DAY_WIDGET_WIDTH - 15.0;
const DAY_WIDGET_ITEM_HEIGHT: f64 = 50.0;

pub struct CalendarWidget {
    read_only_database_service: Rc<dyn ReadOnlyDatabaseService>,
    widget: Flex<DatabaseData>,
}

impl CalendarWidget {
    // pub fn new(widget: Box<dyn Widget<AppState>>, viewmodel: Box<dyn ViewModel>) -> Self {
    //     Self { widget, viewmodel }
    // }

    pub fn new(widget: Flex<DatabaseData>, read_only_database_service: Rc<dyn ReadOnlyDatabaseService>) -> Self {
        let mut answer = Self { widget, read_only_database_service };
        answer.create_day_widgets(MonthData::current());
        answer
    }

    fn create_day_widgets(&mut self, month_data: MonthData) {
        let chores_data = self.read_only_database_service.get_chores_in_interval(month_data.first_day, month_data.last_day).unwrap();

        let current_month_tiles_color = Color::rgb(0.7, 0.9, 1.0);
        let other_tiles_color = Color::rgb(0.8, 0.8, 0.8);
        let get_right_color = |day: &NaiveDate| {
            if day.month() == month_data.first_day.month() {current_month_tiles_color} else {other_tiles_color}
        };

        let important_days = DateUtils::get_important_days(month_data.first_day);
        let mut row = Flex::row();

        important_days.iter().for_each(|day| {
            let container = Container::new(DayWidget::new(*day, chores_data.get(day).cloned().unwrap_or_default()))
                .background( get_right_color(day)) // Darker blue background
                .border(Color::BLACK, 2.0) // Black border around the rectangle
                .fix_size(DAY_WIDGET_WIDTH, DAY_WIDGET_HEIGHT);

            row.add_child(container);
            if day.weekday() == Weekday::Sun {
                self.widget.add_child(mem::replace(&mut row, Flex::row()));
            }
        });
    }
}

impl Widget<DatabaseData> for CalendarWidget {
    delegate!{
        to self.widget {
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DatabaseData, env: &Env);
            fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &DatabaseData, env: &Env);

            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &DatabaseData, data: &DatabaseData, env: &Env);

            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &DatabaseData, env: &Env) -> Size;
            fn paint(&mut self, ctx: &mut PaintCtx, data: &DatabaseData, env: &Env);
        }
    }
}

pub struct DayWidget {
    date: NaiveDate,
    chores: Vec<FullChoreDataRecord>,
    column: Flex<DatabaseData>,
}

impl DayWidget {
    pub fn new(date: NaiveDate, chores: Vec<FullChoreDataRecord>) -> Self {
        let mut answer = Self { date, chores, column: Flex::column() };
        answer.reset();

        answer
    }

    fn reset(&mut self) {
        let label = Label::new(self. date.day().to_string());
        self.column = Flex::column()
            .with_child(
                Padding::new((0.0, 10.0, 0.0, 0.0) , label)
            );

        self.chores.iter().for_each(|rec| {
            let label = Label::new(rec.chore_name().to_string() + "\n" + rec.person_name());
            let container = Container::new(label)
                .background(Color::BLACK)
                .border(Color::BLACK, 2.0)
                .fix_size(DAY_WIDGET_ITEM_WIDTH, DAY_WIDGET_ITEM_HEIGHT);

            self.column.add_child(
                Padding::new((5.0, 5.0, 5.0, 0.0), container));
        });
    }
}

impl Widget<DatabaseData> for DayWidget {
    delegate!{
        to self.column {
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DatabaseData, env: &Env);
            fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &DatabaseData, env: &Env);

            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &DatabaseData, data: &DatabaseData, env: &Env);

            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &DatabaseData, env: &Env) -> Size;
            fn paint(&mut self, ctx: &mut PaintCtx, data: &DatabaseData, env: &Env);
        }
    }
}

// pub fn build_calendar_widget(chores_data: &ChoresData, month_data: MonthData, viewmodel: Box<dyn ViewModel>) -> impl Widget<AppState> {
pub fn build_calendar_widget(read_only_database_service: Rc<dyn ReadOnlyDatabaseService>) -> impl Widget<DatabaseData> {
    let make_label = |label_name| {
        let label =  Label::new(label_name)
            .with_text_size(FONT_SIZE)
            .center();

        return Container::new(Align::centered(label))
            .background( Color::rgb(0.6, 0.8, 0.9)) // Light blue background
            .border(Color::BLACK, 2.0) // Black border around the rectangle
            .fix_size(CELL_WIDTH, CELL_HEIGHT)
    };

    let widget = Flex::column().with_child(
        Flex::row()
            .with_child(make_label("Mon"))
            .with_child(make_label("Tue"))
            .with_child(make_label("Wed"))
            .with_child(make_label("Thu"))
            .with_child(make_label("Fri"))
            .with_child(make_label("Sat"))
            .with_child(make_label("Sun"))
    );

    // CalendarWidget::new(Box::new(widget), viewmodel)
    CalendarWidget::new(widget, read_only_database_service)
}
