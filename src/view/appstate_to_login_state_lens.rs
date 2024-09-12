use druid::Lens;
use crate::view::view_types::{AppState, LoginData};
use crate::view::view_types::AppState::LoginState;

pub struct LoginLens;

impl Lens<AppState, LoginData> for LoginLens {
    fn with<V, F: FnOnce(&LoginData) -> V>(&self, data: &AppState, f: F) -> V {
        if let LoginState(ref login_data) = data {
            f(login_data)
        } else {
            log::info!("Appstate is not in the Login variant at function with");
            f(&LoginData::default())
            // unreachable!("AppState is not in the Login variant")
        }
    }

    fn with_mut<V, F: FnOnce(&mut LoginData) -> V>(&self, data: &mut AppState, f: F) -> V {
        if let LoginState(ref mut login_data) = data {
            f(login_data)
        } else {
            log::info!("Appstate is not in the Login variant at function with_mut");
            f(&mut LoginData::default())
            // unreachable!("AppState is not in the Login variant")
        }
    }
}