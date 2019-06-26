use crate::fields::cardinality::parse_east_west_indicator;
use crate::fields::cardinality::parse_north_south_indicator;
use crate::fields::cardinality::EastWest;
use crate::fields::cardinality::NorthSouth;
use crate::fields::units::ensure_meter;
use crate::fields::units::parse_degree;
use crate::fields::units::parse_dilution_of_precision;
use crate::fields::units::parse_meter;
use crate::fields::units::parse_num_satellites;
use crate::fields::units::parse_quality;
use crate::fields::units::parse_second;
use crate::fields::units::parse_station;
use crate::fields::units::parse_time;
use crate::fields::units::Degree;
use crate::fields::units::Fix;
use crate::fields::units::Meter;
use crate::fields::units::Second;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GGAMessage {
    pub time: Option<NaiveTime>,
    pub lat: Option<Degree>,
    pub ns: NorthSouth,
    pub lon: Option<Degree>,
    pub ew: EastWest,
    pub quality: Fix,
    pub num_sv: Option<u8>,
    pub hdop: Option<f64>,
    pub alt: Option<Meter>,
    pub sep: Option<Meter>,
    pub diff_age: Option<Second>,
    pub diff_station: Option<u8>,
}

pub fn parse_gga(input: &str) -> IResult<&str, GGAMessage> {
    let (
        remaining,
        (time, lat, ns, lon, ew, quality, num_sv, hdop, alt, _, sep, _, diff_age, diff_station),
    ) = tuple((
        parse_time,
        parse_degree,
        parse_north_south_indicator,
        parse_degree,
        parse_east_west_indicator,
        parse_quality,
        parse_num_satellites,
        parse_dilution_of_precision,
        parse_meter,
        ensure_meter,
        parse_meter,
        ensure_meter,
        parse_second,
        parse_station,
    ))(input)?;
    Ok((
        remaining,
        GGAMessage {
            time,
            lat,
            ns,
            lon,
            ew,
            quality,
            num_sv,
            hdop,
            alt,
            sep,
            diff_age,
            diff_station,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gga() {
        let input = "092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,";
        let expected = Ok((
            "",
            GGAMessage {
                time: Some(NaiveTime::from_hms_milli(09, 27, 25, 00)),
                lat: Some(Degree(47.1711399)),
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391590)),
                ew: EastWest::East,
                quality: Fix::AutonomousGNSSFix,
                num_sv: Some(8),
                hdop: Some(1.01),
                alt: Some(Meter(499.6)),
                sep: Some(Meter(48.)),
                diff_age: None,
                diff_station: None,
            },
        ));

        assert_eq!(expected, parse_gga(input));
    }
}
