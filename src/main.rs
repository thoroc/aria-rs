mod cli;
mod duration;

use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use chrono_tz::Tz;
use clap::Parser;
use date_component::date_component;
use log::{debug, info};

use cli::Cli;
use duration::duration_since;

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
