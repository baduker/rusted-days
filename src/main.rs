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

    // let current_year = Utc::now().year();
    // let current_month = Utc::now().month();

    let total_days = Utc::now() - date;
    println!("Result:\n{} days have rusted away ¯\\_(ツ)_/¯", total_days.num_days());

    // let years = current_year - date.year();
    // let months = current_month as i64 - date.month() as i64;
    //
    // let current_month_days = get_days_in_month(current_year, current_month);
    // let month_days_date = get_days_in_month(date.year(), date.month());

    // let days = Utc::now().day() as i64 - date.day() as i64;
    // println!("Timespan:\n{} year(s), {} month(s), {} day(s)", years, months as i64, month_days_date - current_month_days);

    let num_weekday_today = Utc::now()
        .weekday()
        .num_days_from_sunday();

    let num_weekday_date = date
        .weekday()
        .num_days_from_sunday();

    let passing_week_days = (num_weekday_today as i64 - num_weekday_date as i64).abs();
    println!("{} weeks and {} day(s)", total_days.num_weeks(), passing_week_days);
    println!("{:.2} years", total_days.num_days() as f32 / 365_f32);
}