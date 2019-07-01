use crate::fields::cardinality::*;
use crate::fields::distance::*;
use crate::fields::identity::*;
use crate::fields::parameter::*;
use crate::fields::time::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Global positioning system fix data
pub struct GGAMessage {
    /// UTC time
    pub time: Option<NaiveTime>,
    /// Latitude
    pub lat: Option<Degree>,
    /// North/South indicator
    pub ns: NorthSouth,
    /// Longitude
    pub lon: Option<Degree>,
    /// East/West indicator
    pub ew: EastWest,
    /// Quality indicator for position fix
    pub quality: Fix,
    /// Number of satellites used
    pub num_sv: Option<u8>,
    /// Horizontal Dilution of Precision
    pub hdop: Option<f64>,
    /// Altitude above mean sea level
    pub alt: Option<Meter>,
    /// Geoid separation
    pub sep: Option<Meter>,
    /// Age of differential corrections
    pub diff_age: Option<Second>,
    /// ID of station providing differential corrections
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
