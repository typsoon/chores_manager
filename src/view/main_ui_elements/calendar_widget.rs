use crate::model::types::FullChoreDataRecord;
use crate::view::utils::DateUtils;
use crate::view::view_types::AppState;
use crate::view::view_types::MonthData;
use chrono::{Datelike, NaiveDate, Weekday};
use delegate::delegate;
use druid::widget::{Align, Container, Flex, Label, Padding};
use druid::{BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt};
use std::mem;

const CELL_WIDTH: f64 = 250.0;
const CELL_HEIGHT: f64 = 100.0;
const DAY_WIDGET_HEIGHT: f64 = 145.0;
const FONT_SIZE: f64 = 24.0;

pub struct CalendarWidget {
    widget: Flex<AppState>,
}

impl CalendarWidget {
    // pub fn new(widget: Box<dyn Widget<AppState>>, viewmodel: Box<dyn ViewModel>) -> Self {
    //     Self { widget, viewmodel }
    // }

    pub fn new(widget: Flex<AppState>) -> Self {
        let mut answer = Self { widget };
        answer.create_day_widgets(MonthData::current());
        answer
    }

    fn create_day_widgets(&mut self, month_data: MonthData) {
        let current_month_tiles_color = Color::rgb(0.7, 0.9, 1.0);
        let other_tiles_color = Color::rgb(0.8, 0.8, 0.8);
        let get_right_color = |day: &NaiveDate| {
            if day.month() == month_data.first_day.month() {current_month_tiles_color} else {other_tiles_color}
        };

        let important_days = DateUtils::get_important_days(month_data.first_day);
        let mut row = Flex::row();

        important_days.iter().for_each(|day| {
            let container = Container::new(DayWidget::new(*day))
                .background( get_right_color(day)) // Darker blue background
                .border(Color::BLACK, 2.0) // Black border around the rectangle
                .fix_size(CELL_WIDTH, DAY_WIDGET_HEIGHT);

            row.add_child(container);
            if day.weekday() == Weekday::Sun {
                self.widget.add_child(mem::replace(&mut row, Flex::row()));
            }
        });
    }
}

impl Widget<AppState> for CalendarWidget {
    // fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {

    // }

    // fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
    //
    // }

    delegate!{
        to self.widget {
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env);
            fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env);
            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env);
            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size;
            fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env);
        }
    }
}

pub struct DayWidget {
    chores: Vec<FullChoreDataRecord>,
    date: NaiveDate,
    column: Flex<AppState>,
}

impl DayWidget {
    pub fn new(date: NaiveDate) -> Self {
        let label = Label::new(date.day().to_string());
        let column = Flex::column()
            .with_child(
            Padding::new((0.0, 10.0, 0.0, 0.0) , label)
        );

        Self { chores: vec![], date, column }
    }

    // pub fn get_chores(&self, chores_data: &ChoresData) -> Vec<FullChoreDataRecord> {
    //     chores_data.get(&self.date).cloned().unwrap_or_default()
    // }
}

impl Widget<AppState> for DayWidget {
    // fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
    //
    //     self.widget.layout(ctx, bc, data, env)
    // }

    delegate!{
        to self.column {
            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size;
            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env);
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env);
            fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env);
            fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env);
        }
    }
}

// pub fn build_calendar_widget(chores_data: &ChoresData, month_data: MonthData, viewmodel: Box<dyn ViewModel>) -> impl Widget<AppState> {
pub fn build_calendar_widget() -> impl Widget<AppState> {
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
    CalendarWidget::new(widget)
}
