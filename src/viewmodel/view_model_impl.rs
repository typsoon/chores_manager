use crate::model::psql_database_service_impl::create_psql_database_service;
use crate::model::traits::{DatabaseService, ReadOnlyDatabaseService};
use crate::model::types::ChoreTypeRecord;
use crate::model::types::CompletedChoreData;
use crate::model::types::{
    ChoresData, Credentials, DatabaseError, OneTimeChoreRecord, PersonRecord, ScheduledChoreRecord,
};
use crate::viewmodel::view_model_traits::ViewModel;
use chrono::NaiveDate;
use delegate::delegate;

struct ViewModelImpl {
    database_service: Box<dyn DatabaseService>,
}

// #[allow(clippy::result_unit_err)]
pub fn create_view_model(credentials: Credentials) -> Result<impl ViewModel + 'static, ()> {
    match create_psql_database_service(credentials) {
        Ok(database_service) => Ok(ViewModelImpl { database_service }),
        Err(_) => Err(()),
    }
}

impl ReadOnlyDatabaseService for ViewModelImpl {
    delegate! {
        to self.database_service {
            fn get_chores_in_interval(&self, since: NaiveDate, until: NaiveDate) -> Result<ChoresData, DatabaseError>;
            fn get_people(&self) -> Result<Vec<PersonRecord>, DatabaseError>;
            fn get_chores(&self) -> Result<Vec<ChoreTypeRecord>, DatabaseError>;
        }
    }
}

impl DatabaseService for ViewModelImpl {
    delegate! {
        to self.database_service {
            fn add_scheduled_chore(&mut self, scheduled_chore_record: ScheduledChoreRecord);
            fn add_one_time_chore(&mut self, one_time_chore_record: OneTimeChoreRecord);
            fn add_person(&mut self, person_record: PersonRecord);
            fn add_chore_type(&mut self, chore_type_record: ChoreTypeRecord);
            fn complete_chore(&mut self, completed_chore_data: CompletedChoreData);
        }
    }
}

impl ViewModel for ViewModelImpl {}
