use chrono::{Datelike, Days, Local, Months, NaiveDate, Weekday};
use druid::Data;

#[derive(Clone, Debug, Data, PartialEq, Eq, Hash, Copy)]
pub struct MonthData {
    #[data(eq)]
    first_day: NaiveDate,
    #[data(eq)]
    last_day: NaiveDate,
}

impl MonthData {
    pub fn new(first_day: NaiveDate, last_day: NaiveDate) -> Self {
        Self {
            first_day,
            last_day,
        }
    }

    fn get_from_date_like(date_like: impl Datelike) -> Result<Self, ()> {
        let first_day = NaiveDate::from_ymd_opt(date_like.year(), date_like.month(), 1).ok_or(())?;
        let next_month = if date_like.month() == 12 {
            NaiveDate::from_ymd_opt(date_like.year() + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(date_like.year(), date_like.month() + 1, 1)
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

    /// Gets the list of days starting from the monday in the week containing [first_day] and ending before the first day in the month succeeding the month which contains [second_day]
    pub fn get_important_days(&self) -> Vec<NaiveDate> {
        let monday = Self::get_monday(self.first_day);
        let next_month_number = self
            .first_day
            .checked_add_months(Months::new(1))
            .unwrap()
            .month();

        let mut answer: Vec<NaiveDate> = vec![];
        for d in monday.iter_days() {
            if d.month() == next_month_number && d.weekday() == Weekday::Mon {
                break;
            }
            answer.push(d);
        }
        answer
    }

    pub fn current() -> Self {
        Self::get_from_date_like(Local::now()).unwrap()
    }

    pub fn get_prev_month(&self) -> Self {
        Self::get_from_date_like(self.first_day.pred_opt().unwrap_or_default()).unwrap()
    }

    pub fn get_next_month(&self) -> Self {
        Self::get_from_date_like(self.last_day.succ_opt().unwrap_or_default()).unwrap()
    }

    pub fn first_day(&self) -> NaiveDate {
        self.first_day
    }

    pub fn last_day(&self) -> NaiveDate {
        self.last_day
    }
}

impl Default for MonthData {
    fn default() -> Self {
        Self::current()
    }
}
