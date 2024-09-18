use crate::view::main_ui_elements::main_ui_tabs::build_main_ui_tabs;
use crate::view::view_types::app_state::AppState;
use crate::viewmodel::view_model_traits::ViewModel;
use delegate::delegate;
use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget};
use std::rc::Rc;

struct MainUI {
    viewmodel: Rc<dyn ViewModel>,
    widget: Box<dyn Widget<AppState>>,
}

impl MainUI {
    fn new(viewmodel: Rc<dyn ViewModel>, widget: Box<dyn Widget<AppState>>) -> Self {
        Self { viewmodel, widget }
    }
}

impl Widget<AppState> for MainUI {

    // fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
    //     // self.widget = Box::new(build_calendar_widget(&self.chores, MonthData::current()));
    //     self.widget.update(ctx, old_data, data, env);
    // }
    //
    // fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
    //     // self.widget = Box::new(build_calendar_widget(&self.chores, DateUtils::get_month_date_range(Local::now()).unwrap()));
    //     self.widget.layout(ctx, bc, data, env)
    // }

    delegate! {
        to self.widget {
            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env);
            fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env);
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env);
            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size;
            fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env);
        }
    }
}

// pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> MainUI {
pub fn build_main_ui(viewmodel: impl ViewModel+'static) -> impl Widget<AppState> {
    // let calendar_widget = build_calendar_widget(chores_data);
    let viewmodel_rc = Rc::new(viewmodel);
    let tabs = build_main_ui_tabs(viewmodel_rc.clone());
    // let column = Flex::column().with_flex_spacer(1.).with_child(tabs).with_flex_spacer(1.).center();
    MainUI::new(viewmodel_rc, Box::new(tabs))
}