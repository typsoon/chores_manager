mod model;
mod view;
mod viewmodel;

use crate::view::login_screen::build_login_screen;
use crate::view::view_types::AppState;
use druid::{AppLauncher, PlatformError, WindowDesc};
use view::window_manager::WindowManager;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_login_screen())
        .title("Chores Manager");

    AppLauncher::with_window(main_window)
        .delegate(WindowManager::new())
        .log_to_console()
        .launch(AppState::new())
}