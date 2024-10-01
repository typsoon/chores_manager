use crate::model::types::{ChoresData, Credentials};
use crate::view::utils::date_utils::MonthData;
use crate::view::view_types::app_state::AppState::{LoginState, MainState};
use crate::view::view_types::wrappers::{
    ChoreTypeRecordWrapper, ChoresDataKeyVal, FullChoreDataWrapper, FullDayData, ImportantDays,
    NaiveDateWrapper, PersonRecordWrapper,
};
use crate::viewmodel::view_model_traits::ViewModel;
use chrono::{Datelike, Days};
use druid::{Data, Lens};
use std::default::Default;
use std::env;
use std::sync::Arc;

#[derive(Clone, Data)]
pub enum AppState {
    LoginState(LoginData),
    MainState(MainStateData),
}

impl AppState {
    pub fn move_to_main_state(&mut self, viewmodel: &dyn ViewModel) {
        // *self = Main(MainState { chores_data: Default::default() });
        let mut database_data = DatabaseData::default();
        database_data.change_month(MonthData::current(), viewmodel);
        *self = MainState(MainStateData {
            database_data,
            input_data: Default::default(),
        });
    }

    // TODO: try to remove this method
    pub fn get_login_data(&self) -> &LoginData {
        if let LoginState(ref login_data) = self {
            login_data
        } else {
            unreachable!("You shouldn't be calling this function when not in login state");
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        LoginState(LoginData::default())
    }
}

#[derive(Clone, Data, Lens)]
pub struct MainStateData {
    database_data: DatabaseData,
    input_data: MainStateInputData,
}

impl MainStateData {
    pub fn update_data(&mut self, viewmodel: &dyn ViewModel) {
        self.database_data.update_data(viewmodel);
    }

    pub fn change_month(&mut self, month_data: MonthData, viewmodel: &dyn ViewModel) {
        self.database_data.change_month(month_data, viewmodel);
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

        dotenv::dotenv().ok();
        if let (Ok(username), Ok(password)) =
            (env::var("DATABASE_USERNAME"), env::var("DATABASE_PASSWORD"))
        {
            init_username = username;
            init_password = password;
        } else {
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

#[derive(Clone, Data, Lens, Default)]
pub struct DatabaseData {
    #[data(eq)]
    chores_data: ChoresData,
    people: Arc<Vec<PersonRecordWrapper>>,
    chores: Arc<Vec<ChoreTypeRecordWrapper>>,
    #[data(eq)]
    month_data: MonthData,
}

impl DatabaseData {
    fn change_month(&mut self, month_data: MonthData, viewmodel: &dyn ViewModel) {
        self.month_data = month_data;
        self.update_data(viewmodel);
    }

    fn update_data(&mut self, viewmodel: &dyn ViewModel) {
        self.chores_data = viewmodel
            .get_chores_in_interval(
                self.month_data
                    .first_day()
                    .checked_sub_days(Days::new(7))
                    .unwrap(),
                self.month_data
                    .last_day()
                    .checked_add_days(Days::new(7))
                    .unwrap(),
            )
            .unwrap();
        self.people = Arc::new(
            viewmodel
                .get_people()
                .unwrap()
                .into_iter()
                .map(PersonRecordWrapper::new)
                .collect(),
        );

        self.chores = Arc::new(
            viewmodel
                .get_chores()
                .unwrap()
                .into_iter()
                .map(ChoreTypeRecordWrapper::new)
                .collect(),
        );
    }

    pub fn get_important_days(&self) -> ImportantDays {
        let important_days = self.month_data.get_important_days();

        important_days
            .iter()
            .map(|x| {
                let chores = self
                    .chores_data
                    .get(x)
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|x| FullChoreDataWrapper::new(x.clone()))
                    .collect();

                FullDayData::new(
                    self.people.clone(),
                    self.chores.clone(),
                    ChoresDataKeyVal::new(
                        NaiveDateWrapper::new(*x),
                        chores,
                        self.month_data.first_day().month(),
                    ),
                )
            })
            .collect()
    }

    pub fn get_month_data(&self) -> &MonthData {
        &self.month_data
    }
}

#[derive(Clone, Data, Lens, Default)]
pub struct MainStateInputData {
    added_person_name: String,
    added_chore_type_name: String,
    added_chore_name: String,
    added_chore_description: String,
}

impl MainStateInputData {
    pub fn get_added_person_name(&self) -> &str {
        &self.added_person_name
    }

    pub fn get_added_chore_type_name(&self) -> &str {
        &self.added_chore_type_name
    }

    pub fn get_added_chore_description(&self) -> &str {
        &self.added_chore_description
    }
}

pub struct LoginDataLens;

impl Lens<AppState, LoginData> for LoginDataLens {
    fn with<V, F: FnOnce(&LoginData) -> V>(&self, data: &AppState, f: F) -> V {
        if let LoginState(ref login_data) = data {
            f(login_data)
        } else {
            log::info!("Appstate is not in the Login variant at function with");
            f(&Default::default())
            // unreachable!("AppState is not in the Login variant")
        }
    }

    fn with_mut<V, F: FnOnce(&mut LoginData) -> V>(&self, data: &mut AppState, f: F) -> V {
        if let LoginState(ref mut login_data) = data {
            f(login_data)
        } else {
            log::info!("Appstate is not in the Login variant at function with_mut");
            f(&mut Default::default())
            // unreachable!("AppState is not in the Login variant")
        }
    }
}

pub struct MainStateLens;

impl Lens<AppState, MainStateData> for MainStateLens {
    fn with<V, F: FnOnce(&MainStateData) -> V>(&self, data: &AppState, f: F) -> V {
        if let MainState(ref main_state_data) = data {
            f(main_state_data)
        } else {
            unreachable!("AppState not in main state")
        }
    }

    fn with_mut<V, F: FnOnce(&mut MainStateData) -> V>(&self, data: &mut AppState, f: F) -> V {
        if let MainState(ref mut main_state_data) = data {
            f(main_state_data)
        } else {
            unreachable!("AppState not in main state")
        }
    }
}

pub struct ImportantWeeksNewLens;

impl Lens<DatabaseData, ImportantDays> for ImportantWeeksNewLens {
    fn with<V, F: FnOnce(&ImportantDays) -> V>(&self, data: &DatabaseData, f: F) -> V {
        f(&data.get_important_days())
    }

    fn with_mut<V, F: FnOnce(&mut ImportantDays) -> V>(&self, data: &mut DatabaseData, f: F) -> V {
        f(&mut data.get_important_days())
    }
}
