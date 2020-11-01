use std::io;
use std::io::Write;
use chrono::{NaiveDateTime, DateTime, Utc, Datelike, NaiveDate};

fn main() {
    let prompt = "What's your date (DD/MM/YYYY): ";
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

#[allow(dead_code)]
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
    // let days_in_month = get_days_in_month(Utc::now().year(), Utc::now().month());
    let days = date.day() - Utc::now().day();

    println!("Timespan:\n{} year(s), {} month(s), {} day(s)", years, months, days);
    println!("{} weeks", total_days.num_weeks());
    println!("{:.2} years", total_days.num_days() as f32 / 365_f32);
}