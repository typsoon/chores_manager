use crate::model::types::Credentials;
use crate::view::view_types::AppState::{LoginState, Main};
use dotenv::dotenv;
use druid::{Data, Lens, Selector};
use std::env;

#[derive(Clone, Data)]
pub enum AppState {
    LoginState(LoginData),
    Main(MainState)
}

impl AppState {
    pub fn new() -> Self {
        LoginState(LoginData::default())
    }

    pub fn move_to_main_state(&mut self) {
        *self = Main(MainState {});
    }
}
//
// impl Data for FullChoreDataRecord {
//     fn same(&self, other: &Self) -> bool {
//         self.eq(&other)
//     }
// }

#[derive(Clone, Default, Data)]
pub struct MainState {

}

impl MainState {
    // pub fn get_chores_for_day(&self, date: &NaiveDate) -> Vec<ViewFullChoreDataRecord> {
    //     self.chores.clone().get(date).cloned().unwrap_or_default()
    // }
    //
    // pub fn set_chores(&mut self, chores: HashMap<NaiveDate, Vec<FullChoreDataRecord>>) {
    //     self.chores = chores.into_iter().map(ViewFullChoreDataRecord::from).collect();
    // }
}

#[derive(Clone, Data, Lens)]
pub struct LoginData {
    username: String,
    password: String,
    remember: bool,
}

impl LoginData {
    pub fn get_credentials(&self) -> Credentials {
        Credentials(self.username.clone(), self.password.clone())
    }

    #[allow(dead_code)]
    pub fn get_remember(&self) -> bool {
        self.remember
    }
}

impl Default for LoginData {
    fn default() -> Self {
        let init_username;
        let init_password;
        let init_remember = false;

        dotenv().ok();
        if let (Ok(username), Ok(password)) = (env::var("DATABASE_USERNAME"), env::var("DATABASE_PASSWORD")) {
            init_username = username;
            init_password = password;
        }
        else {
            init_username = "".to_string();
            init_password = "".to_string();
        }

        LoginData {
            username: init_username,
            password: init_password,
            remember: init_remember,
        }
    }
}

pub const LOG_IN: Selector<()> = Selector::new("log-in");