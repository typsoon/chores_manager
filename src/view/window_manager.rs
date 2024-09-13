use crate::view::view_types::{AppState, LOG_IN};
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, WindowDesc};
use crate::view::main_ui_elements::main_ui::build_main_ui;
use crate::model::types::Credentials;
use crate::view::view_types::AppState::LoginState;
use crate::viewmodel::view_model_impl::create_view_model;

const MAIN_WINDOW_WIDTH: f64 = 1800.0;
const MAIN_WINDOW_HEIGHT: f64 = 875.0;

pub struct WindowManager {}

impl WindowManager {
    pub fn new() -> Self {
        Self {}
    }

    fn try_log_in(credentials: Credentials, delegate_ctx: &mut DelegateCtx, app_state: &mut AppState) -> bool {
        if let Ok(mut viewmodel) = create_view_model(credentials) {
            delegate_ctx.submit_command(druid::commands::CLOSE_ALL_WINDOWS);

            app_state.move_to_main_state(&mut viewmodel);
            let main_window = WindowDesc::new(build_main_ui(viewmodel))
                .title("Chores Manager")
                .window_size((MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT));

            delegate_ctx.new_window(main_window);
            true
        }
        else {
            false
        }
    }
}


impl AppDelegate<AppState> for WindowManager {
    fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        log::debug!("{:?}", cmd);
        if cmd.is(LOG_IN) {
            if let LoginState(login_data) = data {
                if Self::try_log_in(login_data.get_credentials(), ctx, data) {
                    Handled::Yes
                }
                else {
                    Handled::No
                }
            }
            else {
                unreachable!("AppState should be in the LoginState variant")
            }
        }
        else {
            Handled::No
        }
    }
}