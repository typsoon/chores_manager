use crate::model::types::FullChoreDataRecord;
use crate::view::utils::DateUtils;
use crate::view::view_types::AppState;
use crate::viewmodel::view_model_traits::ViewModel;
use chrono::{Local, NaiveDate};
use delegate::delegate;
use druid::im::{hashmap, HashMap};
use druid::widget::{Flex, Label};
use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt};

type ChoresData = HashMap<NaiveDate, Vec<FullChoreDataRecord>>;

struct MainUI {
    viewmodel: Box<dyn ViewModel>,
    widget: Box<dyn Widget<AppState>>,
    days_widgets: Vec<DayWidget>,
    chores: ChoresData,
}

impl MainUI {
    fn new(viewmodel: Box<dyn ViewModel>, widget: Box<dyn Widget<AppState>>) -> Self {
        Self { viewmodel, widget, days_widgets: vec![], chores: hashmap![] }
    }
}

impl Widget<AppState> for MainUI {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            if let AppState::Main(_main_state) = data {
                let month_data = DateUtils::get_month_date_range(Local::now()).unwrap();
                self.chores.clear();

                let queried_chores = self.viewmodel.get_chores_in_interval(month_data.first_day, month_data.last_day);
                self.chores.extend(queried_chores.unwrap());
            }
            else {
                unreachable!("App is not in the Main state")
            }
        }
        self.widget.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.widget.update(ctx, old_data, data, env);
    }

    delegate! {
        to self.widget {
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env);
            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size;
            fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env);
        }
    }
}

struct DayWidget {
    date: NaiveDate,
    widget: Box<dyn Widget<AppState>>,
}

impl DayWidget {
    pub fn new(date: NaiveDate, widget: Box<dyn Widget<AppState>>) -> Self {
        Self { date, widget }
    }

    pub fn get_chores(&self, chores_data: &ChoresData) -> Vec<FullChoreDataRecord> {
        chores_data.get(&self.date).cloned().unwrap_or_default()
    }
}

// pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> MainUI {
pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> impl Widget<AppState> {
    let label = Label::new("OOOO").padding(5.0).center();

    MainUI::new(viewmodel,  Box::new(Flex::column().with_child(label)))
}