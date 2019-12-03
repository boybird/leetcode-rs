use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, Timelike, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_add_signed(Duration::days(1))
}

fn main() {
    let now = Utc::now();
    println!("{}", now);


    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);
    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("overflow"),
    }
    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }

    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let jp_time_zone = FixedOffset::east(9 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("local time now is {}", local_time);
    println!("utc time now is {}", utc_time);
    println!("jp time now is {}", utc_time.with_timezone(&jp_time_zone));
    println!("rio time now is {}", utc_time.with_timezone(&rio_timezone));

    let (is_pm, hour) = now.hour12();
    println!(
        "The current Utc time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );

    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The Current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}
