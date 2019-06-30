use crate::parser_utils::*;
use chrono::{NaiveDate, NaiveTime};
use nom::IResult;

// ddmmyy arbitrary parsing it as if we always were in the 21st centuary
pub fn parse_date(input: &str) -> IResult<&str, Option<NaiveDate>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, date_str) = parse_string(input)?;
    let maybe_date = if let Ok(raw_ymd) = str::parse::<u32>(date_str) {
        let day = raw_ymd / 10_000;
        let month = (raw_ymd - day * 10_000) / 100;
        let year = raw_ymd - day * 10_000 - month * 100;
        if year > 99 {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::TooLarge)));
        }
        NaiveDate::from_ymd_opt(year as i32 + 2000, month, day)
    } else {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
    };

    Ok((remaining, maybe_date))
}

// 235503.01
// 125027
pub fn parse_time(input: &str) -> IResult<&str, Option<NaiveTime>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, time_str) = parse_string(input)?;

    let splitted: Vec<&str> = time_str.split('.').collect();

    let maybe_time = match (splitted.get(0), splitted.get(1)) {
        (Some(hms), Some(milis)) => {
            if let Ok(raw_hms) = str::parse::<u32>(hms) {
                let hours = raw_hms / 10_000;
                let minutes = (raw_hms - hours * 10_000) / 100;
                let seconds = raw_hms - hours * 10_000 - minutes * 100;
                if let Ok(raw_milis) = str::parse::<u32>(milis) {
                    // Time is provided with two decimals
                    NaiveTime::from_hms_milli_opt(hours, minutes, seconds, raw_milis * 10)
                } else {
                    return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
                }
            } else {
                return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
            }
        }
        (Some(hms), None) => {
            if let Ok(raw_hms) = str::parse::<u32>(hms) {
                let hours = raw_hms / 100;
                let minutes = (raw_hms - hours) / 100;
                let seconds = raw_hms - hours - minutes;
                NaiveTime::from_hms_opt(hours, minutes, seconds)
            } else {
                return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
            }
        }
        _ => None,
    };

    Ok((remaining, maybe_time))
}
