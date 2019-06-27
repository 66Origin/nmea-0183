use crate::fields::cardinality::*;
use crate::fields::units::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GLLMessage {
    pub lat: Option<Degree>,
    pub ns: NorthSouth,
    pub lon: Option<Degree>,
    pub ew: EastWest,
    pub time: Option<NaiveTime>,
    pub status: Status,
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
