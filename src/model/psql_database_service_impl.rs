use crate::model::traits::DatabaseService;
use crate::model::traits::ReadOnlyDatabaseService;
use crate::model::types::Credentials;
use crate::model::types::{ChoreRecord, FullChoreDataRecord, PersonRecord, ScheduledChoreRecord};
use chrono::NaiveDate;
use diesel::sql_types::Date;
use diesel::{sql_query, Connection, ConnectionError, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

struct PSQLDatabaseService {
    connection: PgConnection,
}

fn establish_connection(credentials: Credentials) ->  Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        credentials.0, credentials.1, host, database_name
    );

    PgConnection::establish(&database_url)
}

pub fn create_psql_database_service(credentials: Credentials) -> Result<Box<dyn DatabaseService>, ()> {
    match establish_connection(credentials) {
        Ok(connection) => Ok(Box::new(PSQLDatabaseService { connection })),
        Err(_) => Err(()),
    }
}

impl ReadOnlyDatabaseService for PSQLDatabaseService {
    fn get_chores_in_interval(&mut self, since: NaiveDate, until: NaiveDate) -> Result<HashMap<NaiveDate, Vec<FullChoreDataRecord>>, ()> {
        let fetched_data = sql_query("SELECT * FROM AllChoresView WHERE date_of BETWEEN $1 AND $2")
            .bind::<Date, _>(since)
            .bind::<Date, _>(until)
            .load::<FullChoreDataRecord>(&mut self.connection);

        match fetched_data {
            Ok(data) => {
                let mut answer: HashMap<NaiveDate, Vec<FullChoreDataRecord>> = HashMap::new();
                data.iter().for_each(|x| {
                    answer.entry(x.date_of.date())
                        .or_insert_with(Vec::new)
                        .push(x.clone());
                });
                Ok(answer)
            }
            Err(_) => Err(()),
        }
    }
}

#[allow(dead_code)]
impl DatabaseService for PSQLDatabaseService {
    fn add_scheduled_chore(&self, scheduled_chore_record: ScheduledChoreRecord) {
        todo!()
    }

    fn add_one_time_chore(&self, one_time_chore_record: ChoreRecord) {
        todo!()
    }

    fn add_person(&self, person_record: PersonRecord) {
        todo!()
    }

    fn add_chore(&self, chore_record: ChoreRecord) {
        todo!()
    }
}