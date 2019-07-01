use crate::fields::cardinality::*;
use crate::fields::distance::*;
use crate::fields::parameter::*;
use crate::fields::time::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Latitude and longitude, with time of position fix and status
pub struct GLLMessage {
    /// Latitude
    pub lat: Option<Degree>,
    /// North/South indicator
    pub ns: NorthSouth,
    /// Longitude
    pub lon: Option<Degree>,
    /// East/West indicator
    pub ew: EastWest,
    /// UTC time
    pub time: Option<NaiveTime>,
    /// Data validity status
    pub status: Status,
    /// Positioning mode
    pub pos_mode: Fix,
}

pub fn parse_gll(input: &str) -> IResult<&str, GLLMessage> {
    let (remaining, (lat, ns, lon, ew, time, status, pos_mode)) = tuple((
        parse_degree,
        parse_north_south_indicator,
        parse_degree,
        parse_east_west_indicator,
        parse_time,
        parse_status,
        parse_pos_mode,
    ))(input)?;
    Ok((
        remaining,
        GLLMessage {
            lat,
            ns,
            lon,
            ew,
            time,
            status,
            pos_mode,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gll() {
        let input = "4717.11364,N,00833.91565,E,092321.00,A,A";
        let expected = Ok((
            "",
            GLLMessage {
                lat: Some(Degree(47.171136399999995)), // floats ¯\_(ツ)_/¯
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391565)),
                ew: EastWest::East,
                time: Some(NaiveTime::from_hms(9, 23, 21)),
                status: Status::DataValid,
                pos_mode: Fix::AutonomousGNSSFix,
            },
        ));

        assert_eq!(expected, parse_gll(input));
    }
}
