use crate::fields::cardinality::*;
use crate::fields::parse_float;
use crate::fields::units::*;
use chrono::naive::{NaiveDate, NaiveTime};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct VLWMessage {
    pub twd: Option<f64>,
    pub twd_unit: Option<WaterDistanceUnit>,
    pub wd: Option<f64>,
    pub wd_unit: Option<WaterDistanceUnit>,
    pub tgd: Option<f64>,
    pub tgd_unit: Option<WaterDistanceUnit>,
    pub gd: Option<f64>,
    pub gd_unit: Option<WaterDistanceUnit>,
}

pub fn parse_vlw(input: &str) -> IResult<&str, VLWMessage> {
    let (remaining, (twd, twd_unit, wd, wd_unit, tgd, tgd_unit, gd, gd_unit)) = tuple((
        parse_float,
        parse_water_distance_unit,
        parse_float,
        parse_water_distance_unit,
        parse_float,
        parse_water_distance_unit,
        parse_float,
        parse_water_distance_unit,
    ))(input)?;
    Ok((
        remaining,
        VLWMessage {
            twd,
            twd_unit,
            wd,
            wd_unit,
            tgd,
            tgd_unit,
            gd,
            gd_unit,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vlw() {
        let input = ",N,,N,15.8,N,1.2,N";
        let expected = Ok((
            "",
            VLWMessage {
                twd: None,
                twd_unit: Some(WaterDistanceUnit::NauticalMile),
                wd: None,
                wd_unit: Some(WaterDistanceUnit::NauticalMile),
                tgd: Some(15.8),
                tgd_unit: Some(WaterDistanceUnit::NauticalMile),
                gd: Some(1.2),
                gd_unit: Some(WaterDistanceUnit::NauticalMile),
            },
        ));

        assert_eq!(expected, parse_vlw(input));
    }
}
