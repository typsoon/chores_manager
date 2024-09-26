mod model;
mod view;
mod viewmodel;

use crate::view::configure_env::configure_env;
use crate::view::login_screen::build_login_screen;
use crate::view::view_types::app_state::LoginDataLens;
use druid::{AppLauncher, PlatformError, WidgetExt, WindowDesc};
use tracing_subscriber::fmt::Subscriber;
use view::view_types::app_state::AppState;
use view::window_manager::WindowManager;

fn main() -> Result<(), PlatformError> {
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // let login_screen = build_login_screen();
    let main_window =
        // WindowDesc::new(login_screen.lens(LoginDataLens)).title("Chores Manager");
        WindowDesc::new(build_login_screen().lens(LoginDataLens)).title("Chores Manager");

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| configure_env(env))
        .delegate(WindowManager {})
        // .log_to_console()
        .launch(AppState::default())
}
