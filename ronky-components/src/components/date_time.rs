use bevy::prelude::*;

pub trait DateTimeTrait: chrono::Datelike + chrono::Timelike {}
impl<T: chrono::Datelike + chrono::Timelike> DateTimeTrait for T {}

pub trait DateTimeProvider: Send + Sync {
    /// Returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).
    fn year(&self) -> i32;

    /// Returns the absolute year number starting from 1 with a boolean flag,
    /// which is false when the year predates the epoch (BCE/BC) and true otherwise (CE/AD).
    #[inline]
    fn year_ce(&self) -> (bool, u32) {
        let year = self.year();
        if year < 1 {
            (false, (1 - year) as u32)
        } else {
            (true, year as u32)
        }
    }

    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    fn month(&self) -> u32;

    /// Returns the month number starting from 0.
    ///
    /// The return value ranges from 0 to 11.
    fn month0(&self) -> u32;

    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    fn day(&self) -> u32;

    /// Returns the day of month starting from 0.
    ///
    /// The return value ranges from 0 to 30. (The last day of month differs by months.)
    fn day0(&self) -> u32;

    /// Returns the day of year starting from 1.
    ///
    /// The return value ranges from 1 to 366. (The last day of year differs by years.)
    fn ordinal(&self) -> u32;

    /// Returns the day of year starting from 0.
    ///
    /// The return value ranges from 0 to 365. (The last day of year differs by years.)
    fn ordinal0(&self) -> u32;

    /// Returns the day of week.
    fn weekday(&self) -> chrono::Weekday;

    /// Returns the ISO week.
    fn iso_week(&self) -> chrono::IsoWeek;

    /// Returns the hour number from 0 to 23.
    fn hour(&self) -> u32;

    /// Returns the hour number from 1 to 12 with a boolean flag,
    /// which is false for AM and true for PM.
    #[inline]
    fn hour12(&self) -> (bool, u32) {
        let hour = self.hour();
        let mut hour12 = hour % 12;
        if hour12 == 0 {
            hour12 = 12;
        }
        (hour >= 12, hour12)
    }

    /// Returns the minute number from 0 to 59.
    fn minute(&self) -> u32;

    /// Returns the second number from 0 to 59.
    fn second(&self) -> u32;

    /// Returns the number of nanoseconds since the whole non-leap second.
    /// The range from 1,000,000,000 to 1,999,999,999 represents
    /// the [leap second](./naive/struct.NaiveTime.html#leap-second-handling).
    fn nanosecond(&self) -> u32;
}

pub struct DateTimeValue<T>(T);

#[derive(Component)]
pub struct DateTime(pub Box<dyn DateTimeProvider>);

impl<T: DateTimeTrait + Send + Sync> DateTimeProvider for DateTimeValue<T> {
    fn year(&self) -> i32 {
        self.0.year()
    }

    fn month(&self) -> u32 {
        self.0.month()
    }

    fn month0(&self) -> u32 {
        self.0.month0()
    }

    fn day(&self) -> u32 {
        self.0.day()
    }

    fn day0(&self) -> u32 {
        self.0.day0()
    }

    fn ordinal(&self) -> u32 {
        self.0.ordinal()
    }

    fn ordinal0(&self) -> u32 {
        self.0.ordinal0()
    }

    fn weekday(&self) -> chrono::Weekday {
        self.0.weekday()
    }

    fn iso_week(&self) -> chrono::IsoWeek {
        self.0.iso_week()
    }

    fn hour(&self) -> u32 {
        self.0.hour()
    }

    fn minute(&self) -> u32 {
        self.0.minute()
    }

    fn second(&self) -> u32 {
        self.0.second()
    }

    fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }
}
