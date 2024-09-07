pub mod traits  {
    use chrono::{DateTime, Duration, Utc};

    trait ReadOnlyDatabaseService {
        fn get_chores_in_interval(&self, date_from :DateTime<Utc> , date_to : DateTime<Utc>);

        fn add_scheduled_task(&self, interval : Duration, date_from :DateTime<Utc> , date_to : DateTime<Utc>);
    }
}