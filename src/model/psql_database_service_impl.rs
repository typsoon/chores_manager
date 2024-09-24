use crate::model::traits::DatabaseService;
use crate::model::traits::ReadOnlyDatabaseService;
use crate::model::types::ChoreTypeRecord;
use crate::model::types::{
    ChoreRecord, DatabaseError, FullChoreDataRecord, PersonRecord, ScheduledChoreRecord,
};
use crate::model::types::{ChoresData, Credentials};
use chrono::NaiveDate;
use diesel::r2d2::{ConnectionManager, Error, ManageConnection, Pool};
use diesel::sql_types::{Date, VarChar};
use diesel::{sql_query, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
struct PSQLDatabaseService {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

fn establish_connection(
    credentials: Credentials,
) -> Result<Pool<ConnectionManager<PgConnection>>, Error> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        credentials.0, credentials.1, host, database_name
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    manager.connect()?;
    Ok(Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool"))
    // PgConnection::establish(&database_url)
}

pub fn create_psql_database_service(
    credentials: Credentials,
) -> Result<Box<dyn DatabaseService>, ()> {
    match establish_connection(credentials) {
        Ok(connection_pool) => Ok(Box::new(PSQLDatabaseService { connection_pool })),
        Err(_) => Err(()),
    }
}

impl ReadOnlyDatabaseService for PSQLDatabaseService {
    fn get_chores_in_interval(
        &self,
        since: NaiveDate,
        until: NaiveDate,
    ) -> Result<ChoresData, DatabaseError> {
        let fetched_data = sql_query("SELECT * FROM AllChoresView WHERE date_of BETWEEN $1 AND $2")
            .bind::<Date, _>(since)
            .bind::<Date, _>(until)
            .load::<FullChoreDataRecord>(&mut self.connection_pool.get().unwrap());

        match fetched_data {
            Ok(data) => {
                let mut answer: ChoresData = ChoresData::new();
                data.iter().for_each(|x| {
                    answer
                        .entry(x.date_of().date())
                        .or_default()
                        .push(x.clone());
                });
                Ok(answer)
            }
            Err(er) => Err(DatabaseError::Error(er)),
        }
    }

    fn get_people(&self) -> Result<Vec<PersonRecord>, DatabaseError> {
        let fetched_data = sql_query("SELECT * FROM PeopleView")
            .load::<PersonRecord>(&mut self.connection_pool.get().unwrap());

        match fetched_data {
            Ok(data) => Ok(data),
            Err(err) => Err(DatabaseError::Error(err)),
        }
    }

    fn get_chores(&self) -> Result<Vec<ChoreTypeRecord>, DatabaseError> {
        let fetched_data = sql_query("SELECT * FROM ChoresView")
            .load::<ChoreTypeRecord>(&mut self.connection_pool.get().unwrap());

        match fetched_data {
            Ok(data) => Ok(data),
            Err(err) => Err(DatabaseError::Error(err)),
        }
    }
}

#[allow(dead_code, unused_variables)]
impl DatabaseService for PSQLDatabaseService {
    fn add_scheduled_chore(&mut self, scheduled_chore_record: ScheduledChoreRecord) {
        todo!()
    }

    fn add_one_time_chore(&mut self, one_time_chore_record: ChoreRecord) {
        todo!()
    }

    fn add_person(&mut self, person_record: PersonRecord) {
        sql_query("INSERT INTO PeopleView(person_name) VALUES ($1)")
            .bind::<VarChar, _>(person_record.person_name())
            .execute(&mut self.connection_pool.get().unwrap())
            .unwrap();
    }

    fn add_chore_type(&mut self, chore_type_record: ChoreTypeRecord) {
        sql_query("INSERT INTO ChoresView(chore_name, chore_description) VALUES ($1, $2)")
            .bind::<VarChar, _>(chore_type_record.chore_name())
            .bind::<VarChar, _>(chore_type_record.chore_description())
            .execute(&mut self.connection_pool.get().unwrap())
            .unwrap();
    }
}
