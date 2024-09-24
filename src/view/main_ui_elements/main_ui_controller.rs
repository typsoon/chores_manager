use crate::view::main_ui_elements::main_ui_tabs::build_main_ui_tabs;
use crate::view::view_types::app_state::MainStateData;
use crate::view::view_types::selectors::{ADD_CHORE_TYPE, ADD_PERSON, CHANGE_MONTH};
use crate::viewmodel::view_model_traits::ViewModel;
use druid::widget::Controller;
use druid::{Env, Event, EventCtx, Widget, WidgetExt};
use std::ops::Deref;

struct MainUIController {
    viewmodel: Box<dyn ViewModel>,
}

impl MainUIController {
    pub fn new(viewmodel: Box<dyn ViewModel>) -> Self {
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
                data.change_month(month_data.clone(), self.viewmodel.deref());
            }

            if let Some(chore_type_record) = cmd.get(ADD_CHORE_TYPE) {
                self.viewmodel.add_chore_type(chore_type_record.clone());
                data.update_data(self.viewmodel.deref());
            }

            if let Some(person_record) = cmd.get(ADD_PERSON) {
                self.viewmodel.add_person(person_record.clone());
                data.update_data(self.viewmodel.deref());
            }

            // ctx.request_update();
        }

        child.event(ctx, event, data, env);
    }
}

// pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> MainUI {
pub fn build_main_ui(viewmodel: Box<dyn ViewModel>) -> impl Widget<MainStateData> {
    // let calendar_widget = build_calendar_widget(chores_data);
    let tabs = build_main_ui_tabs();
    // let column = Flex::column().with_flex_spacer(1.).with_child(tabs).with_flex_spacer(1.).center();
    Box::new(tabs).controller(MainUIController::new(viewmodel))
}
