use crate::view::configure_env::{
    LOGIN_SCREEN_TEXT_BOX_WIDTH, LOGIN_SCREEN_TEXT_FIELD_FONT, LOGIN_SCREEN_VERTICAL_WIDGET_SPACING,
};
use crate::view::view_types::app_state::LoginData;
use crate::view::view_types::selectors::LOG_IN;
use druid::widget::{Button, Checkbox, Flex, TextBox};
use druid::{Env, EventCtx, LifeCycleCtx, UnitPoint, Widget, WidgetExt};

#[allow(dead_code)]
pub fn build_login_screen() -> impl Widget<LoginData> {
    Flex::column()
        .on_added(|column, life_cycle_ctx: &mut LifeCycleCtx, _, _: &Env| {
            column.add_child(
                TextBox::new()
                    .with_placeholder("Username")
                    .with_font(LOGIN_SCREEN_TEXT_FIELD_FONT)
                    .fix_width(LOGIN_SCREEN_TEXT_BOX_WIDTH)
                    .center()
                    .lens(LoginData::username),
            );
            column.add_spacer(LOGIN_SCREEN_VERTICAL_WIDGET_SPACING);
            column.add_child(
                TextBox::new()
                    .with_placeholder("Password")
                    .with_font(LOGIN_SCREEN_TEXT_FIELD_FONT)
                    .fix_width(LOGIN_SCREEN_TEXT_BOX_WIDTH)
                    .center()
                    .lens(LoginData::password),
            );
            column.add_spacer(LOGIN_SCREEN_VERTICAL_WIDGET_SPACING);
            column.add_child(Button::new("Log in").on_click(
                |ctx: &mut EventCtx, _data: &mut LoginData, _env| {
                    ctx.submit_command(LOG_IN);
                },
            ));
            column.add_spacer(LOGIN_SCREEN_VERTICAL_WIDGET_SPACING);
            column.add_child(
                Checkbox::new("Remember credentials")
                    .lens(LoginData::remember)
                    .center()
                    .padding(8.0),
            );

            life_cycle_ctx.submit_command(LOG_IN);
        })
        .align_vertical(UnitPoint::CENTER)
}
