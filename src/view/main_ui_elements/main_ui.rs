use crate::view::main_ui_elements::main_ui_tabs::build_main_ui_tabs;
use crate::view::view_types::app_state::MainStateData;
use crate::viewmodel::view_model_traits::ViewModel;
use druid::widget::Controller;
use druid::{Widget, WidgetExt};
use std::rc::Rc;

struct MainUIController {
    #[allow(dead_code)]
    viewmodel: Rc<dyn ViewModel>,
}

impl MainUIController {
    pub fn new(viewmodel: Rc<dyn ViewModel>) -> Self {
        Self { viewmodel }
    }
}

impl<W: Widget<MainStateData>> Controller<MainStateData, W> for MainUIController {}

// pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> MainUI {
pub fn build_main_ui(viewmodel: Rc<impl ViewModel + 'static>) -> impl Widget<MainStateData> {
    // let calendar_widget = build_calendar_widget(chores_data);
    let tabs = build_main_ui_tabs();
    // let column = Flex::column().with_flex_spacer(1.).with_child(tabs).with_flex_spacer(1.).center();
    Box::new(tabs).controller(MainUIController::new(viewmodel))
}
