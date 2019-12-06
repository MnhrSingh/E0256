extern crate chrono;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    let rfc2822_fmt = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("rfc2822 format for date and time......{}", rfc2822_fmt);

    let rfc3339_fmt = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("rfc3339 format for date and time......{}", rfc3339_fmt);

    let customized = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("date and time in cutomized form.......{}", customized);

    let time = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("time is...............................{}", time);

    let date = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("date is ..............................{}", date);

    let without_tmzn = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("date and time without time zone.......{}", without_tmzn);

    Ok(())
}
