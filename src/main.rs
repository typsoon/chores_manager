mod model;
mod view;
mod viewmodel;

use crate::view::login_screen::build_login_screen;
use crate::view::view_types::app_state::LoginDataLens;
use druid::{AppLauncher, PlatformError, WidgetExt, WindowDesc};
use view::view_types::app_state::AppState;
use view::window_manager::WindowManager;

fn main() -> Result<(), PlatformError> {
    let main_window =
        WindowDesc::new(build_login_screen().lens(LoginDataLens)).title("Chores Manager");

    AppLauncher::with_window(main_window)
        .delegate(WindowManager {})
        .log_to_console()
        .launch(AppState::new())
}
