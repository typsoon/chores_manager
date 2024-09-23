use crate::model::types::{ChoresData, Credentials};
use crate::view::view_types::app_state::AppState::{LoginState, MainState};
use crate::view::view_types::utils::MonthData;
use crate::view::view_types::wrappers::{
    ChoreTypeRecordWrapper, ChoresDataKeyVal, FullChoreDataWrapper, ImportantWeeks,
    NaiveDateWrapper, PersonRecordWrapper,
};
use crate::viewmodel::view_model_traits::ViewModel;
use chrono::{Datelike, Weekday};
use druid::im::{vector, Vector};
use druid::{Data, Lens};
use std::default::Default;
use std::env;
use std::rc::Rc;

#[derive(Clone, Data)]
pub enum AppState {
    LoginState(LoginData),
    MainState(MainStateData),
}

impl AppState {
    pub fn new() -> Self {
        LoginState(LoginData::default())
    }

    pub fn move_to_main_state(&mut self, viewmodel: Rc<impl ViewModel + 'static>) {
        // *self = Main(MainState { chores_data: Default::default() });
        let mut database_data = DatabaseData::new(viewmodel);
        database_data.update_data(&MonthData::current());
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

#[derive(Clone, Data, Lens)]
pub struct MainStateData {
    database_data: DatabaseData,
    input_data: MainStateInputData,
}

impl MainStateData {
    pub fn update_data(&mut self, month_data: &MonthData) {
        self.database_data.update_data(month_data);
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

#[derive(Clone, Data, Lens)]
pub struct DatabaseData {
    #[data(ignore)]
    #[lens(ignore)]
    viewmodel: Rc<dyn ViewModel>,
    #[data(eq)]
    chores_data: ChoresData,
    people: Vector<PersonRecordWrapper>,
    chores: Vector<ChoreTypeRecordWrapper>,
    #[data(eq)]
    month_data: MonthData,
}

impl DatabaseData {
    fn update_data(&mut self, month_data: &MonthData) {
        self.month_data = month_data.clone();

        self.chores_data = self
            .viewmodel
            .get_chores_in_interval(month_data.first_day(), month_data.last_day())
            .unwrap();
        self.people = self
            .viewmodel
            .get_people()
            .unwrap()
            .into_iter()
            .map(PersonRecordWrapper::new)
            .collect();

        self.chores = self
            .viewmodel
            .get_chores()
            .unwrap()
            .into_iter()
            .map(ChoreTypeRecordWrapper::new)
            .collect();
    }

    pub fn get_important_weeks(&self) -> ImportantWeeks {
        let mut answer = vector![];
        let important_days = self.month_data.get_important_days();
        important_days.iter().for_each(|x| {
            if x.weekday() == Weekday::Mon {
                answer.push_back(vector![]);
            }

            let chores = self
                .chores_data
                .get(x)
                .unwrap_or(&vec![])
                .into_iter()
                .map(|x| FullChoreDataWrapper::new(x.clone()))
                .collect();

            answer
                .iter_mut()
                .last()
                .unwrap()
                .push_back(ChoresDataKeyVal::new(
                    NaiveDateWrapper::new(*x),
                    chores,
                    self.month_data.first_day().month().clone(),
                ));
        });

        answer
    }

    pub fn new(viewmodel: Rc<dyn ViewModel>) -> Self {
        Self {
            viewmodel,
            chores_data: Default::default(),
            people: Default::default(),
            chores: Default::default(),
            month_data: Default::default(),
        }
    }

    // pub fn get_chores_data(&self) -> &ChoresData {
    //     &self.chores_data
    // }

    pub fn get_month_data(&self) -> &MonthData {
        &self.month_data
    }
}

#[derive(Clone, Data, Lens, Default)]
pub struct MainStateInputData {
    added_person_name: String,
    added_chore_type_name: String,
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
