use chrono::{NaiveDate, ParseResult};

pub struct Date {
    date: NaiveDate
}

impl Date {

    pub fn from_str(date: &str) -> ParseResult<Date> {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").map(|date| Date { date })
    }

}