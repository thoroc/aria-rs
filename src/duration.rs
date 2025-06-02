use chrono::DateTime;
use chrono::Utc;
use log::debug;

pub struct Duration {
    pub years: i64,
    pub months: i64,
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
}

pub fn duration_since(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Duration {
    // let now: DateTime<Utc> = Utc::now();
    debug!("To date:   {}", end_date.format("%Y-%m-%d %H:%M:%S"));
    debug!("From date: {}", start_date.format("%Y-%m-%d %H:%M:%S"));
    let duration = end_date.signed_duration_since(start_date);

    let years = duration.num_days() / 365;
    let remaining_days = duration.num_days() % 365;
    debug!("Years: {}, Remaining days: {}", years, remaining_days);

    let months = remaining_days / 30;
    let remaining_days = remaining_days % 30;
    debug!("Months: {}, Remaining days: {}", months, remaining_days);

    let weeks = remaining_days / 7;
    let remaining_days = remaining_days % 7;
    debug!("Weeks: {}, Remaining days: {}", weeks, remaining_days);

    let years_in_days = years * 365;
    let months_in_days = months * 30;
    let weeks_in_days = weeks * 7;
    let days = duration.num_days() - weeks_in_days - months_in_days - years_in_days;
    debug!("Days: {}", days);

    let years_in_hours = years_in_days * 24;
    let months_in_hours = months_in_days * 24;
    let weeks_in_hours = weeks_in_days * 24;
    let days_in_hours = days * 24;
    let hours =
        duration.num_hours() - days_in_hours - weeks_in_hours - months_in_hours - years_in_hours;
    debug!("Hours: {}", hours);

    let years_in_minutes = years_in_hours * 60;
    let months_in_minutes = months_in_hours * 60;
    let weeks_in_minutes = weeks_in_hours * 60;
    let days_in_minutes = days_in_hours * 60;
    let hours_in_minutes = hours * 60;
    let minutes = duration.num_minutes()
        - hours_in_minutes
        - days_in_minutes
        - weeks_in_minutes
        - months_in_minutes
        - years_in_minutes;
    debug!("Minutes: {}", duration.num_minutes());

    debug!(
        "Years: {}, Months: {}, Weeks: {}, Days: {}, Hours: {}, Minutes: {}",
        years, months, weeks, days, hours, minutes
    );

    return Duration {
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
    };
}
