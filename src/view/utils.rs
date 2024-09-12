use chrono::{Datelike, NaiveDate};

pub struct DateUtils;

pub struct MonthData {
    pub first_day: NaiveDate,
    pub last_day: NaiveDate,
}

impl MonthData {
    pub fn new(first_day: NaiveDate, last_day: NaiveDate) -> Self {
        Self { first_day, last_day }
    }
}

impl DateUtils {
    pub fn get_month_date_range(month_data: impl Datelike) -> Result<MonthData, ()> {
        let first_day = NaiveDate::from_ymd_opt(month_data.year(), month_data.month(), 1).ok_or(())?;
        let next_month = if month_data.month() == 12 {
            NaiveDate::from_ymd_opt(month_data.year() + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(month_data.year(), month_data.month() + 1, 1)
        }.ok_or(())?;
        let last_day = next_month.pred_opt().ok_or(())?; // Subtract one day to get the last day of the current month

        Ok(MonthData::new(first_day, last_day))
    }
}