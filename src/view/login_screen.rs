use crate::view::view_types::selectors::LOG_IN;
use delegate::delegate;
use druid::widget::{Button, Checkbox, Flex, TextBox};
use druid::{BoxConstraints, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UnitPoint, UpdateCtx, Widget, WidgetExt};
use crate::view::view_types::app_state::{AppState, LoginData, LoginLens};

struct LoginLayoutConfig;

impl LoginLayoutConfig {
    pub const VERTICAL_WIDGET_SPACING: f64 = 10.0;
    pub const TEXT_BOX_WIDTH: f64 = 200.0;
    pub const FONT_SIZE: f64 = 18.0;
}

pub struct LoginScreen {
    widget: Box<dyn Widget<AppState>>,
}

impl LoginScreen {
    fn new(widget: Box<dyn Widget<AppState>>) -> Self {
        Self { widget }
    }
}

impl Widget<AppState> for LoginScreen {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            ctx.submit_command(LOG_IN);
        }
        self.widget.lifecycle(ctx, event, data, env);
    }

    delegate! {
        to self.widget {
            fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env);
            fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env);
            fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size;
            fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env);
        }
    }
}

pub fn build_login_screen() -> impl Widget<AppState> {
    // let title_font = FontDescriptor::new(FontFamily::SYSTEM_UI)
    //     .with_weight(FontWeight::BOLD)
    //     .with_size(40.0);
    let text_field_font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(LoginLayoutConfig::FONT_SIZE);

    // let label = Label::new("Log in").
    //     with_font(title_font)
    //     .center()
    //     .padding(10.0);

    let username_box = TextBox::new()
        .with_placeholder("Username")
        .with_font(text_field_font.clone())
        .fix_width(LoginLayoutConfig::TEXT_BOX_WIDTH)
        .center()
        .lens(LoginData::username);

    let password_box = TextBox::new()
        .with_placeholder("Password")
        .with_font(text_field_font.clone())
        .fix_width(LoginLayoutConfig::TEXT_BOX_WIDTH)
        .center()
        .lens(LoginData::password);

    let log_in_button = Button::new("Log in")
        .on_click(|ctx: &mut EventCtx, _data: &mut LoginData, _env| {
            ctx.submit_command(LOG_IN);
        });


    let remember_radio_box = Checkbox::new("Remember credentials")
        .lens(LoginData::remember)
        .center()
        .padding(8.0);


    LoginScreen::new(Box::new(Flex::column()
        .with_child(username_box)
        .with_spacer(LoginLayoutConfig::VERTICAL_WIDGET_SPACING)
        .with_child(password_box)
        .with_spacer(LoginLayoutConfig::VERTICAL_WIDGET_SPACING)
        .with_child(remember_radio_box)
        .with_spacer(LoginLayoutConfig::VERTICAL_WIDGET_SPACING)
        .with_child(log_in_button)
        .with_spacer(LoginLayoutConfig::VERTICAL_WIDGET_SPACING)
        .lens(LoginLens)))
        .align_vertical(UnitPoint::CENTER)
}