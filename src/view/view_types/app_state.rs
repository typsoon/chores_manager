use druid::{Data, Lens};
use crate::model::types::{ChoresData, Credentials};
use druid::im::Vector;
use std::env;
use crate::model::traits::ReadOnlyDatabaseService;
use crate::viewmodel::view_model_traits::ViewModel;
use crate::view::view_types::utils::MonthData;
use crate::view::view_types::wrappers::{ChoreTypeRecordWrapper, PersonRecordWrapper};

#[derive(Clone, Default, Data, Lens)]
pub struct AppState {
    login_data: Option<LoginData>,
    #[data(eq)]
    chores_data: ChoresData,
    #[data(eq)]
    month_data: MonthData,
    people: Vector<PersonRecordWrapper>,
    chores: Vector<ChoreTypeRecordWrapper>
    // main_state: MainState
}

impl AppState {
    pub fn new() -> Self {
        let mut answer = Self::default();
        answer.login_data = Some(LoginData::default());
        answer
    }

    pub fn update_data(&mut self, month_data: MonthData, viewmodel: &mut dyn ViewModel) {
        self.month_data = month_data;
        self.chores_data = viewmodel.get_chores_in_interval(self.month_data.first_day, self.month_data.last_day).unwrap();
        self.people = viewmodel.get_people().unwrap().into_iter()
            .map(|x| PersonRecordWrapper::new(x))
            .collect();

        self.chores = viewmodel.get_chores().unwrap().into_iter()
            .map(|x| {ChoreTypeRecordWrapper::new(x)})
            .collect();
        // self.main_state.update_chores_data(MonthData::current(), viewmodel);
    }

    pub fn move_to_main_state(&mut self, viewmodel: &mut dyn ViewModel){
        // *self = Main(MainState { chores_data: Default::default() });
        self.login_data = None;
        self.update_data(MonthData::current(), viewmodel);
    }

    pub fn get_login_data(&self) -> &Option<LoginData> {
        &self.login_data
    }
}

pub struct LoginLens;

impl Lens<AppState, LoginData> for LoginLens {
    fn with<V, F: FnOnce(&LoginData) -> V>(&self, data: &AppState, f: F) -> V {
        if let Some(ref login_data) = data.login_data {
            f(login_data)
        } else {
            log::info!("Appstate is not in the Login variant at function with");
            f(&LoginData::default())
            // unreachable!("AppState is not in the Login variant")
        }
    }

    fn with_mut<V, F: FnOnce(&mut LoginData) -> V>(&self, data: &mut AppState, f: F) -> V {
        if let Some(ref mut login_data) = data.login_data {
            f(login_data)
        } else {
            log::info!("Appstate is not in the Login variant at function with_mut");
            f(&mut LoginData::default())
            // unreachable!("AppState is not in the Login variant")
        }
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

// #[derive(Clone, Default, Data)]
// pub struct MainState {
//     #[data(eq)]
//     chores_data: ChoresData,
//     #[data(eq)]
//     month_data: MonthData,
//     #[data(eq)]
//     people: Vec<PersonRecord>,
//     #[data(eq)]
//     chores: Vec<ChoreRecord>
// }

// impl MainState {
//     // pub fn get_chores_for_day(&self, date: &NaiveDate) -> Vec<FullChoreDataRecord> {
//     //     self.chores_data.get(date).cloned().unwrap_or_default()
//     // }
//
//     pub fn update_chores_data(&mut self, month_data: MonthData, viewmodel: &mut dyn ViewModel) {
//         self.month_data = month_data;
//         self.chores_data = viewmodel.get_chores_in_interval(self.month_data.first_day, self.month_data.last_day).unwrap();
//     }
//
//     // pub fn chores_data(&self) -> &ChoresData {
//     //     &self.chores_data
//     // }
//     //
//     // pub fn month_data(&self) -> &MonthData {
//     //     &self.month_data
//     // }
//
//     pub fn new(chores_data: ChoresData, month_data: MonthData, people: Vec<PersonRecord>, chores: Vec<ChoreRecord>) -> Self {
//         Self { chores_data, month_data, people, chores }
//     }
// }
