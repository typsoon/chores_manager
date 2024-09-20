use chrono::{Datelike, Days, Local, Months, NaiveDate, Weekday};
pub struct DateUtils;

impl DateUtils {
    pub fn get_month_date_range(month_data: impl Datelike) -> Result<MonthData, ()> {
        let first_day =
            NaiveDate::from_ymd_opt(month_data.year(), month_data.month(), 1).ok_or(())?;
        let next_month = if month_data.month() == 12 {
            NaiveDate::from_ymd_opt(month_data.year() + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(month_data.year(), month_data.month() + 1, 1)
        }
        .ok_or(())?;
        let last_day = next_month.pred_opt().ok_or(())?; // Subtract one day to get the last day of the current month

        Ok(MonthData::new(first_day, last_day))
    }

    fn get_monday(day: NaiveDate) -> NaiveDate {
        let weekday_num = day.weekday().number_from_monday();
        assert!(weekday_num > 0);
        day.checked_sub_days(Days::new((weekday_num - 1) as u64))
            .unwrap()
    }

    /// Gets the list of days starting from the monday in the week containing [day] and ending before the first day in the month succeeding the month which contains [day]
    pub fn get_important_days(day: NaiveDate) -> Vec<NaiveDate> {
        let monday = Self::get_monday(day);
        let next_month_number = day.checked_add_months(Months::new(1)).unwrap().month();

        let mut answer: Vec<NaiveDate> = vec![];
        for d in monday.iter_days() {
            if d.month() == next_month_number && d.weekday() == Weekday::Mon {
                break;
            }
            answer.push(d);
        }
        answer
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MonthData {
    pub first_day: NaiveDate,
    pub last_day: NaiveDate,
}

impl MonthData {
    pub fn new(first_day: NaiveDate, last_day: NaiveDate) -> Self {
        Self {
            first_day,
            last_day,
        }
    }

    pub fn current() -> Self {
        DateUtils::get_month_date_range(Local::now()).unwrap()
    }
}

impl Default for MonthData {
    fn default() -> Self {
        Self::current()
    }
}
