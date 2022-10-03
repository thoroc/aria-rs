# Usage:

```
$ cargo run -- --datetime "2022-05-13 12:48:00" --timezone "Europe/London" -v --name Aria
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/aria-rs --datetime '2022-05-13 12:48:00' --timezone Europe/London -v --name Aria`
[2022-10-03T08:10:44Z DEBUG aria_rs] DateTime: 2022-05-13 12:48:00
[2022-10-03T08:10:44Z DEBUG aria_rs] Local: 2022-05-13 12:48:00
[2022-10-03T08:10:44Z DEBUG aria_rs] UTC: 2022-05-13 12:48:00
[2022-10-03T08:10:44Z DEBUG aria_rs] TimeZone: Europe/London
[2022-10-03T08:10:44Z DEBUG aria_rs] Europe/London: 2022-05-13 12:48:00
[2022-10-03T08:10:44Z DEBUG aria_rs] Now: 2022-10-03 08:10:44
[2022-10-03T08:10:44Z DEBUG aria_rs] To date:   2022-10-03 08:10:44
[2022-10-03T08:10:44Z DEBUG aria_rs] From date: 2022-05-13 12:48:00
[2022-10-03T08:10:44Z DEBUG aria_rs] Years: 0, Remaining days: 142
[2022-10-03T08:10:44Z DEBUG aria_rs] Months: 4, Remaining days: 22
[2022-10-03T08:10:44Z DEBUG aria_rs] Weeks: 3, Remaining days: 1
[2022-10-03T08:10:44Z DEBUG aria_rs] Days: 1
[2022-10-03T08:10:44Z DEBUG aria_rs] Hours: 19
[2022-10-03T08:10:44Z DEBUG aria_rs] Minutes: 205642
[2022-10-03T08:10:44Z DEBUG aria_rs] Years: 0, Months: 4, Weeks: 3, Days: 1, Hours: 19, Minutes: 22
[2022-10-03T08:10:44Z INFO  aria_rs] Aria was born 0 year(s) 4 month(s) 3 week(s) 1 day(s) 19 hour(s) 22 minute(s) ago
[2022-10-03T08:10:44Z INFO  aria_rs] DateComponent { year: 0, month: 4, day: 20, interval_seconds: 12338564, interval_minutes: 205642, interval_hours: 3427, interval_day: 142, invert: false }
[2022-10-03T08:10:44Z DEBUG aria_rs] seconds: 142
[2022-10-03T08:10:44Z DEBUG aria_rs] minutes: 142
[2022-10-03T08:10:44Z DEBUG aria_rs] hours: 142
[2022-10-03T08:10:44Z DEBUG aria_rs] days: 142
```

# Motivation

I wanted to practice some rust and thought creating a simple cli tool to check when my little one was born exactely would be fun.