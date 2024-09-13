use crate::model::types::FullChoreDataRecord;
use crate::model::types::{ChoresData, Credentials};
use crate::view::utils::DateUtils;
use crate::view::view_types::AppState::{LoginState, Main};
use crate::viewmodel::view_model_traits::ViewModel;
use chrono::{Local, NaiveDate};
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

    pub fn move_to_main_state(&mut self, viewmodel: &mut Box<dyn ViewModel>) {
        // *self = Main(MainState { chores_data: Default::default() });
        let mut main_state = MainState::new(Default::default(), Default::default());
        main_state.update_chores_data(MonthData::current(), viewmodel);
        *self = Main(main_state);
    }
}

#[derive(Clone, Default, Data)]
pub struct MainState {
    #[data(eq)]
    chores_data: ChoresData,
    #[data(eq)]
    month_data: MonthData
}

impl MainState {
    pub fn get_chores_for_day(&self, date: &NaiveDate) -> Vec<FullChoreDataRecord> {
        self.chores_data.get(date).cloned().unwrap_or_default()
    }

    pub fn update_chores_data(&mut self, month_data: MonthData, viewmodel: &mut Box<dyn ViewModel>) {
        self.month_data = month_data;
        self.chores_data = viewmodel.get_chores_in_interval(self.month_data.first_day, self.month_data.last_day).unwrap();
    }

    pub fn chores_data(&self) -> &ChoresData {
        &self.chores_data
    }

    pub fn month_data(&self) -> &MonthData {
        &self.month_data
    }

    pub fn new(chores_data: ChoresData, month_data: MonthData) -> Self {
        Self { chores_data, month_data }
    }
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

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct MonthData {
    pub first_day: NaiveDate,
    pub last_day: NaiveDate,
}

impl MonthData {
    pub fn new(first_day: NaiveDate, last_day: NaiveDate) -> Self {
        Self { first_day, last_day }
    }

    pub fn current() -> Self {
        DateUtils::get_month_date_range(Local::now()).unwrap()
    }
}