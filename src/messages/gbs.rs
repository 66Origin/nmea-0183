use crate::fields::distance::*;
use crate::fields::time::*;
use crate::parser_utils::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// GNSS Satellite Fault Detection
pub struct GBSMessage {
    /// UTC time to which this RAIM sentencebelongs
    pub time: Option<NaiveTime>,
    /// Expected error in latitude
    pub lat_err: Option<Meter>,
    /// Expected error in longitude
    pub lon_err: Option<Meter>,
    /// Expected error in altitude
    pub alt_err: Option<Meter>,
    /// Satellite ID of most likely failed satellite
    pub sat_prn: Option<u8>,
    /// Probability of missed detection
    pub prob: Option<u8>,
    /// Estimated bias of most likely failed satellite
    pub res: Option<f64>,
    /// Standard deviation of estimated bias
    pub std_dev: Option<f64>,
    /// NMEA defined GNSS System ID
    pub system_id: Option<u8>,
    /// NMEA defined GNSS Signal ID,
    pub signal_id: Option<u8>,
}

pub fn parse_gbs(input: &str) -> IResult<&str, GBSMessage> {
    let (
        remaining,
        (time, lat_err, lon_err, alt_err, sat_prn, prob, res, std_dev, system_id, signal_id),
    ) = tuple((
        parse_time,
        parse_meter,
        parse_meter,
        parse_meter,
        parse_u8,
        parse_u8,
        parse_float,
        parse_float,
        parse_u8,
        parse_u8,
    ))(input)?;
    Ok((
        remaining,
        GBSMessage {
            time,
            lat_err,
            lon_err,
            alt_err,
            sat_prn,
            prob,
            res,
            std_dev,
            system_id,
            signal_id,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gbs() {
        let input = "235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0";
        let expected = Ok((
            "",
            GBSMessage {
                time: Some(NaiveTime::from_hms(23, 54, 58)),
                lat_err: Some(Meter(1.4)),
                lon_err: Some(Meter(1.3)),
                alt_err: Some(Meter(3.1)),
                sat_prn: Some(3),
                prob: None,
                res: Some(-21.4),
                std_dev: Some(3.8),
                system_id: Some(1),
                signal_id: Some(0),
            },
        ));

        assert_eq!(expected, parse_gbs(input));
    }
}
