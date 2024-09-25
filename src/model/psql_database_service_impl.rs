use crate::model::traits::DatabaseService;
use crate::model::traits::ReadOnlyDatabaseService;
use crate::model::types::ChoreTypeRecord;
use crate::model::types::CompletedChoreData;
use crate::model::types::{ChoresData, Credentials};
use crate::model::types::{
    DatabaseError, FullChoreDataRecord, OneTimeChoreRecord, PersonRecord, ScheduledChoreRecord,
};
use chrono::NaiveDate;
use diesel::r2d2::{ConnectionManager, Error, ManageConnection, Pool};
use diesel::sql_types::{Date, Integer, Interval, Timestamp, VarChar};
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
        sql_query("INSERT INTO ScheduledChoresView(person_name, chore_name, chore_interval, date_from, date_to) VALUES ($1, $2, $3, $4, $5)", )
            .bind::<VarChar, _>(scheduled_chore_record.person_name())
            .bind::<VarChar, _>(scheduled_chore_record.chore_name())
            .bind::<Interval, _>(scheduled_chore_record.interval())
            .bind::<Timestamp, _>(scheduled_chore_record.date_from())
            .bind::<Timestamp, _>(scheduled_chore_record.date_to())
            .execute(&mut self.connection_pool.get().unwrap())
            .unwrap();
    }

    fn add_one_time_chore(&mut self, one_time_chore_record: OneTimeChoreRecord) {
        sql_query(
            "INSERT INTO OneTimeChoresView(person_name, chore_name, date_of) VALUES ($1, $2, $3)",
        )
        .bind::<VarChar, _>(one_time_chore_record.person_name())
        .bind::<VarChar, _>(one_time_chore_record.chore_name())
        .bind::<Timestamp, _>(one_time_chore_record.date_of())
        .execute(&mut self.connection_pool.get().unwrap())
        .unwrap();
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

    fn complete_chore(&mut self, completed_chore_data: CompletedChoreData) {
        sql_query(
            "INSERT INTO CompletedChoresView(chore_name, iteration, message) VALUES ($1, $2, $3)",
        )
        .bind::<VarChar, _>(completed_chore_data.chore_name())
        .bind::<Integer, _>(completed_chore_data.iteration())
        .bind::<VarChar, _>(completed_chore_data.message())
        .execute(&mut self.connection_pool.get().unwrap())
        .unwrap();
    }
}
