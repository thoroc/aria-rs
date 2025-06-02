use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use chrono_tz::Tz;
use clap::Parser;
use date_component::date_component;
use log::{debug, info};

// create a struct CLI to hold the command line arguments
#[derive(Parser)]
#[clap(name = "demo")]
struct Cli {
    // the name for the event:
    #[clap(short = 'n', long = "name", default_value = "aria")]
    name: String,

    // the date of birth in format YYYY-MM-DD HH:MM:SS (default: current date)
    #[clap(short = 'd', long = "datetime")]
    datetime: Option<String>,

    // the timezone of the date (default: UTC)
    #[clap(short = 't', long = "timezone")]
    timezone: Option<String>,

    // allow for debug flag
    #[clap(short = 'v', long = "verbose")]
    debug: bool,
}

fn main() {
    let args = Cli::parse();
    let debug = args.debug;

    let log_level: String = if debug {
        "debug".to_string()
    } else {
        "info".to_string()
    };

    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, log_level),
    );

    let str_datetime: String = args.datetime.unwrap_or_else(|| {
        // get the current date
        let now = chrono::Local::now();
        // format the date
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    });
    let datetime = NaiveDateTime::parse_from_str(&str_datetime, "%Y-%m-%d %H:%M:%S").unwrap();
    debug!("DateTime: {}", datetime);

    let dt_local: DateTime<Local> = Local.from_local_datetime(&datetime).unwrap();
    debug!("Local: {}", dt_local.format("%Y-%m-%d %H:%M:%S"));

    let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&datetime);
    debug!("UTC: {}", dt_utc.format("%Y-%m-%d %H:%M:%S"));

    let str_timezone: String = args.timezone.unwrap_or_else(|| "UTC".to_string());
    let timezone: Tz = str_timezone.parse().unwrap();
    debug!("TimeZone: {:?}", timezone);

    // change datetime to UTC
    // let datetime_utc = datetime.with_timezone(&timezone);
    // let utc: DateTime<Tz> = dt_local.with_timezone(&timezone);
    // debug!("{}: {}", str_timezone, utc.format("%Y-%m-%d %H:%M:%S"));

    let now: DateTime<Utc> = Utc::now();
    debug!("Now: {}", now.format("%Y-%m-%d %H:%M:%S"));

    let duration = duration_since(dt_utc, now);

    let mut msg = String::new();

    let duration_years = duration.years;
    if duration_years > 0 || debug == true {
        msg.push_str(&format!("{} year(s) ", duration_years));
    }

    let duration_months = duration.months;
    if duration_months > 0 || debug == true {
        msg.push_str(&format!("{} month(s) ", duration_months));
    }

    let duration_weeks = duration.weeks;
    if duration_weeks > 0 || debug == true {
        msg.push_str(&format!("{} week(s) ", duration_weeks));
    }

    let duration_days = duration.days;
    if duration_days > 0 || debug == true {
        msg.push_str(&format!("{} day(s) ", duration_days));
    }

    let duration_hours = duration.hours;
    if duration_hours > 0 || debug == true {
        msg.push_str(&format!("{} hour(s) ", duration_hours));
    }

    let duration_minutes = duration.minutes;
    if duration_minutes > 0 || debug == true {
        msg.push_str(&format!("{} minute(s)", duration_minutes));
    }

    let name = args.name;
    info!("{} was born {} ago", name, msg);

    let date_interval = date_component::calculate(&dt_utc, &now);
    debug!("{:?}", date_interval);

    debug!(
        "seconds: {}",
        date_interval.interval_seconds / (60 * 60 * 24)
    );
    debug!("minutes: {}", date_interval.interval_minutes / (60 * 24));
    debug!("hours: {}", date_interval.interval_hours / 24);
    debug!("days: {}", date_interval.interval_days);
}

struct Duration {
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
}

/// Returns the difference between two `DateTime`s in local `Duration` format.
fn duration_since(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Duration {
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
