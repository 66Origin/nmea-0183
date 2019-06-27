use crate::fields::cardinality::*;
use crate::fields::units::*;
use chrono::naive::{NaiveDate, NaiveTime};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct RMCMessage {
    pub time: Option<NaiveTime>,
    pub status: Status,
    pub lat: Option<Degree>,
    pub ns: NorthSouth,
    pub lon: Option<Degree>,
    pub ew: EastWest,
    pub spd: Option<Knot>,
    pub cog: Option<Degree>,
    pub date: Option<NaiveDate>,
    pub mv: Option<Degree>,
    pub mv_ew: Option<EastWest>,
    pub pos_mode: Fix,
    pub nav_status: NavigationalStatus,
}

pub fn parse_rmc(input: &str) -> IResult<&str, RMCMessage> {
    let (
        remaining,
        (time, status, lat, ns, lon, ew, spd, cog, date, mv, mv_ew, pos_mode, nav_status),
    ) = tuple((
        parse_time,
        parse_status,
        parse_degree,
        parse_north_south_indicator,
        parse_degree,
        parse_east_west_indicator,
        parse_knot,
        parse_raw_degree,
        parse_date,
        parse_degree,
        parse_maybe_east_west_indicator,
        parse_pos_mode,
        parse_navigational_status,
    ))(input)?;
    Ok((
        remaining,
        RMCMessage {
            time,
            status,
            lat,
            ns,
            lon,
            ew,
            spd,
            cog,
            date,
            mv,
            mv_ew,
            pos_mode,
            nav_status,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rmc() {
        let input = "083559.00,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A,V";
        let expected = Ok((
            "",
            RMCMessage {
                time: NaiveTime::from_hms_opt(8, 35, 59),
                status: Status::DataValid,
                lat: Some(Degree(47.1711437)),
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391522)),
                ew: EastWest::East,
                spd: Some(Knot(0.004)),
                cog: Some(Degree(77.52)),
                date: NaiveDate::from_ymd_opt(2002, 12, 09),
                mv: None,
                mv_ew: None,
                pos_mode: Fix::AutonomousGNSSFix,
                nav_status: NavigationalStatus::NotValid,
            },
        ));

        assert_eq!(expected, parse_rmc(input));
    }
}
