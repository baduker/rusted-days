use std::io;
use std::io::Write;
use chrono::{NaiveDateTime, DateTime, Utc, Datelike, NaiveDate};

fn main() {
    let prompt = "What's your date (DD/MM/YYYY)?: ";
    let date = input(prompt).expect("Something went wrong! o.O");
    show_interpretation(parse_date(date));
}

fn input(user_input: &str) -> io::Result<String> {
    print!("{}", user_input);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}

fn parse_date(date: String) -> DateTime<Utc> {
    let naive_date = chrono::NaiveDate::parse_from_str(
        date.as_str(), "%d/%m/%Y"
    ).unwrap();

    let naive_datetime: NaiveDateTime = naive_date
        .and_hms(0, 0, 0);

    DateTime::<Utc>::from_utc(naive_datetime, Utc)
}

fn get_days_in_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

fn show_interpretation(date: DateTime<Utc>) {
    println!("Input interpretation: days since {}", date.format("%A, %B %d, %Y"));

    let total_days = Utc::now() - date;
    println!("Result:\n{} days have rusted away ¯\\_(ツ)_/¯", total_days.num_days());

    let years = Utc::now().year() - date.year();
    let months = Utc::now().month() - date.month();

    if date.day() > Utc::now().day() {
        let days = get_days_in_month(date.year(), date.month()) - date.day() as i64;
            println!("Timespan:\n{} year(s), {} month(s), {} day(s)", years, months - 1, days);
    } else {
        let days = Utc::now().day() - date.day();
            println!("Timespan:\n{} year(s), {} month(s), {} day(s)", years, months, days);
    }

    let num_day_today = Utc::now().weekday().num_days_from_sunday();
    let num_day_date = date.weekday().num_days_from_sunday();

    println!("{} weeks and {} day(s)", total_days.num_weeks(), num_day_today - num_day_date);
    println!("{:.2} years", total_days.num_days() as f32 / 365_f32);
}