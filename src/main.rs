mod model;

use diesel::prelude::*;
use diesel::sql_query;
use dotenv::dotenv;
use model::types::PersonRecord;
use std::env;

fn main() {
    use diesel::prelude::*;

    // dotenv().ok();

    let connection = &mut establish_connection();

    // sql_query("INSERT INTO People(person_name) VALUES ('testaad')").
    //     execute(connection).unwrap();

    let values = sql_query("SELECT person_name FROM peopleView")
        .load::<PersonRecord>(connection)
        .expect("Error loading person records");

    println!("{:?}", values);

    println!("Hello, world!");
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}