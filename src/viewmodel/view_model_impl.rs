use crate::model::psql_database_service_impl::create_psql_database_service;
use crate::model::traits::{DatabaseService, ReadOnlyDatabaseService};
use crate::model::types::{ChoreRecord, ChoresData, Credentials, PersonRecord, ScheduledChoreRecord};
use crate::viewmodel::view_model_traits::ViewModel;
use chrono::NaiveDate;
use delegate::delegate;

struct ViewModelImpl {
    database_service: Box<dyn DatabaseService>,
}

pub fn create_view_model(credentials: Credentials) -> Result<Box<dyn ViewModel>, ()> {
    match create_psql_database_service(credentials) {
        Ok(database_service) => Ok(Box::new(ViewModelImpl { database_service }) as Box<dyn ViewModel>,),
        Err(_) => Err(()),
    }
}

impl ReadOnlyDatabaseService for ViewModelImpl {

    delegate! {
        to self.database_service {
           fn get_chores_in_interval(&self, since: NaiveDate, until: NaiveDate) -> Result<ChoresData, ()>;
        }
    }
}

impl DatabaseService for ViewModelImpl {
    delegate! {
        to self.database_service {
            fn add_scheduled_chore(&mut self, scheduled_chore_record: ScheduledChoreRecord);
            fn add_one_time_chore(&mut self, one_time_chore_record: ChoreRecord);
            fn add_person(&mut self, person_record: PersonRecord);
            fn add_chore(&mut self, chore_record: ChoreRecord);
        }
    }
}

impl ViewModel for ViewModelImpl {}