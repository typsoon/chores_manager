use crate::view::main_ui_elements::main_ui_tabs::build_main_ui_tabs;
use crate::view::view_types::app_state::MainStateData;
use crate::view::view_types::selectors::CHANGE_MONTH;
use crate::viewmodel::view_model_traits::ViewModel;
use druid::widget::Controller;
use druid::{Env, Event, EventCtx, Widget, WidgetExt};
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

impl<W: Widget<MainStateData>> Controller<MainStateData, W> for MainUIController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut MainStateData,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
            if let Some(month_data) = cmd.get(CHANGE_MONTH) {
                data.update_data(month_data);
                ctx.request_update();
            }
        }

        child.event(ctx, event, data, env);
    }
}

// pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> MainUI {
pub fn build_main_ui(viewmodel: Rc<impl ViewModel + 'static>) -> impl Widget<MainStateData> {
    // let calendar_widget = build_calendar_widget(chores_data);
    let tabs = build_main_ui_tabs();
    // let column = Flex::column().with_flex_spacer(1.).with_child(tabs).with_flex_spacer(1.).center();
    Box::new(tabs).controller(MainUIController::new(viewmodel))
}
