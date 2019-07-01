use crate::fields::time::*;
use crate::parser_utils::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Time and Date
pub struct ZDAMessage {
    /// UTC Time
    pub time: Option<NaiveTime>,
    /// UTC day
    pub day: Option<u8>,
    /// UTC month
    pub month: Option<u8>,
    /// UTC year
    pub year: Option<u16>,
    /// Local time zone hours
    pub ltzh: Option<u8>,
    /// Local time zone hours
    pub ltzn: Option<u8>,
}

pub fn parse_zda(input: &str) -> IResult<&str, ZDAMessage> {
    let (remaining, (time, day, month, year, ltzh, ltzn)) = tuple((
        parse_time, parse_u8, parse_u8, parse_u16, parse_u8, parse_u8,
    ))(input)?;
    Ok((
        remaining,
        ZDAMessage {
            time,
            day,
            month,
            year,
            ltzh,
            ltzn,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_zda() {
        let input = "082710.00,16,09,2002,00,00";
        let expected = Ok((
            "",
            ZDAMessage {
                time: Some(NaiveTime::from_hms(8, 27, 10)),
                day: Some(16),
                month: Some(09),
                year: Some(2002),
                ltzh: Some(0),
                ltzn: Some(0),
            },
        ));

        assert_eq!(expected, parse_zda(input));
    }
}
