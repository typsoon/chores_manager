use crate::view::main_ui_elements::main_ui_tabs::build_main_ui_tabs;
use crate::view::view_types::app_state::MainStateData;
use crate::viewmodel::view_model_traits::ViewModel;
use delegate::delegate;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};
use std::rc::Rc;

struct MainUI {
    viewmodel: Rc<dyn ViewModel>,
    widget: Box<dyn Widget<MainStateData>>,
}

impl MainUI {
    fn new(viewmodel: Rc<dyn ViewModel>, widget: Box<dyn Widget<MainStateData>>) -> Self {
        Self { viewmodel, widget }
    }
}

impl Widget<MainStateData> for MainUI {
    delegate! {
        to self.widget {
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MainStateData, env: &Env);

            fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &MainStateData, env: &Env);

            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &MainStateData, data: &MainStateData, env: &Env);

            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &MainStateData, env: &Env) -> Size;

            fn paint(&mut self, ctx: &mut PaintCtx, data: &MainStateData, env: &Env);
        }
    }
}

// pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> MainUI {
pub fn build_main_ui(viewmodel: Rc<impl ViewModel + 'static>) -> impl Widget<MainStateData> {
    // let calendar_widget = build_calendar_widget(chores_data);
    let tabs = build_main_ui_tabs(viewmodel.clone());
    // let column = Flex::column().with_flex_spacer(1.).with_child(tabs).with_flex_spacer(1.).center();
    MainUI::new(viewmodel, Box::new(tabs))
}
