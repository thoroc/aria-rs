use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use chrono_tz::Tz;
use clap::Parser;
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
  let utc: DateTime<Tz> = dt_local.with_timezone(&timezone);
  debug!("{}: {}", str_timezone, utc.format("%Y-%m-%d %H:%M:%S"));

  let timestamp: i64 = utc.timestamp();
  debug!("Timestamp: {}", timestamp);

  let now: DateTime<Utc> = Utc::now();
  debug!("Now: {}", now.format("%Y-%m-%d %H:%M:%S"));

  let now_timestamp: i64 = now.timestamp();
  debug!("Now Timestamp: {}", now_timestamp);

  let duration = now.signed_duration_since(dt_utc);
  // let duration_years = duration.num_days() / 365;
  // let duration_months = duration.num_days() / 30;
  // let duration_weeks = duration.num_weeks();
  // let duration_days = duration.num_days();
  // let duration_hours = duration.num_hours();
  // let duration_minutes = duration.num_minutes();

  debug!("Duration: {}", duration);
  // debug!(
  //   "Years: {}, Months: {}, Weeks: {}, Days: {}, Hours: {}, Minutes {}",
  //   duration_years,
  //   duration_months,
  //   duration_weeks,
  //   duration_days,
  //   duration_hours,
  //   duration_minutes
  // );

  let mut msg = String::new();

  let duration_years = duration.num_days() / 365;
  if duration_years > 0 || debug == true {
    msg.push_str(&format!("{} year(s) ", duration_years));
  }
  debug!("Duration (Years): {}", duration.num_days() / 365);

  let years_in_months = duration_years * 12;
  let duration_months = duration.num_days() / 30 - years_in_months - 1;
  if duration_months > 0 || debug == true {
    msg.push_str(&format!("{} month(s) ", duration_months));
  }
  debug!("Duration in months: {}", duration.num_days() / 30);

  let years_in_weeks = duration_years * 52;
  let months_in_weeks = duration_months * 4;
  let duration_weeks = duration.num_weeks() - years_in_weeks - months_in_weeks - 1;
  if duration_weeks > 0 || debug == true {
    msg.push_str(&format!("{} week(s) ", duration_weeks));
  }
  debug!("Duration (weeks): {}", duration_weeks);

  let years_in_days = duration_years * 365;
  let months_in_days = duration_months * 30;
  let weeks_in_days = duration_weeks * 7;
  let duration_days = duration.num_days() - weeks_in_days - months_in_days - years_in_days - 1;
  if duration_days > 0 || debug == true {
    msg.push_str(&format!("{} day(s) ", duration_days));
  }
  debug!("Duration (days): {}", duration_days);

  let years_in_hours = years_in_days * 24;
  let months_in_hours = months_in_days * 24;
  let weeks_in_hours = weeks_in_days * 24;
  let days_in_hours = (duration_days + 1) * 24;
  let duration_hours =
    duration.num_hours() - days_in_hours - weeks_in_hours - months_in_hours - years_in_hours;
  if duration_hours > 0 || debug == true {
    msg.push_str(&format!("{} hour(s) ", duration_hours));
  }
  debug!("Duration (hours): {}", duration_hours);

  let years_in_minutes = years_in_hours * 60;
  let months_in_minutes = months_in_hours * 60;
  let weeks_in_minutes = weeks_in_hours * 60;
  let days_in_minutes = days_in_hours * 60;
  let hours_in_minutes = duration_hours * 60;
  let duration_minutes = duration.num_minutes()
    - hours_in_minutes
    - days_in_minutes
    - weeks_in_minutes
    - months_in_minutes
    - years_in_minutes;
  if duration_minutes > 0 || debug == true {
    msg.push_str(&format!("{} minute(s)", duration_minutes));
  }
  debug!("Duration (minutes): {}", duration_minutes);

  let name = args.name;
  info!("{} was born {} ago", name, msg);
}
