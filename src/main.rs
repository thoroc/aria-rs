#![allow(unused)]
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use chrono_tz::Tz;
use clap::Parser;

// create a struct CLI to hold the command line arguments
#[derive(Parser)]
#[clap(name = "demo")]
struct Cli {
  // the date of birth in format YYYY-MM-DD HH:MM:SS (default: current date)
  #[clap(short, long)]
  datetime: Option<String>,
  // the timezone of the date (default: UTC)
  #[clap(short, long)]
  timezone: Option<String>,
}

fn main() {
  let args = Cli::parse();
  let str_datetime: String = args.datetime.unwrap_or_else(|| {
    // get the current date
    let now = chrono::Local::now();
    // format the date
    now.format("%Y-%m-%d %H:%M:%S").to_string()
  });
  let datetime = NaiveDateTime::parse_from_str(&str_datetime, "%Y-%m-%d %H:%M:%S").unwrap();
  println!("DateTime: {}", datetime);

  let dt_local: DateTime<Local> = Local.from_local_datetime(&datetime).unwrap();
  println!("Local: {}", dt_local.format("%Y-%m-%d %H:%M:%S"));

  let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&datetime);
  println!("UTC: {}", dt_utc.format("%Y-%m-%d %H:%M:%S"));

  let str_timezone: String = args.timezone.unwrap_or_else(|| "UTC".to_string());
  let timezone: Tz = str_timezone.parse().unwrap();
  println!("TimeZone: {:?}", timezone);

  // change datetime to UTC
  // let datetime_utc = datetime.with_timezone(&timezone);
  let utc: DateTime<Tz> = dt_local.with_timezone(&timezone);
  println!("{}: {}", str_timezone, utc.format("%Y-%m-%d %H:%M:%S"));

  let timestamp: i64 = utc.timestamp();
  println!("Timestamp: {}", timestamp);

  let now: DateTime<Utc> = Utc::now();
  println!("Now: {}", now.format("%Y-%m-%d %H:%M:%S"));

  let now_timestamp: i64 = now.timestamp();
  println!("Now Timestamp: {}", now_timestamp);

  let duration = now.signed_duration_since(dt_utc);
  println!("Duration: {}", duration);
  println!(
    "Weeks: {}, Days: {}, Hours: {}, Minutes: {}",
    duration.num_weeks(),
    duration.num_days(),
    duration.num_hours(),
    duration.num_minutes()
  );

  let mut msg = String::new();

  let duration_weeks = duration.num_weeks();
  if duration_weeks > 0 {
    msg.push_str(&format!("{} week(s) ", duration_weeks));
  }
  println!("Duration (weeks): {}", duration_weeks);

  let weeks_in_days = duration_weeks * 7;
  let duration_days = duration.num_days() - weeks_in_days;
  if duration_days > 0 {
    msg.push_str(&format!("{} day(s) ", duration_days));
  }
  println!("Duration (days): {}", duration_days);

  let days_in_hours = duration_days * 24;
  let weeks_in_hours = weeks_in_days * 24;
  let duration_hours = duration.num_hours() - days_in_hours - weeks_in_hours;
  if duration_hours > 0 {
    msg.push_str(&format!("{} hour(s) ", duration_hours));
  }
  println!("Duration (hours): {}", duration_hours);

  let hours_in_minutes = duration_hours * 60;
  let days_in_minutes = days_in_hours * 60;
  let weeks_in_minutes = weeks_in_hours * 60;
  let duration_minutes =
    duration.num_minutes() - hours_in_minutes - days_in_minutes - weeks_in_minutes;
  if duration_minutes > 0 {
    msg.push_str(&format!("{} minute(s) ", duration_minutes));
  }
  println!("Duration (minutes): {}", duration_minutes);

  println!("Duration: {}", msg);
}
