use crate::fields::speed::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Dual ground/water distance
pub struct VLWMessage {
    /// Total cumulative water distance:
    pub twd: Option<f64>,
    /// Total cumulative water distance units
    pub twd_unit: Option<WaterDistanceUnit>,
    /// Water distance since reset
    pub wd: Option<f64>,
    /// Water distance since reset units
    pub wd_unit: Option<WaterDistanceUnit>,
    /// Total cumulative ground distance
    pub tgd: Option<f64>,
    /// Total cumulative ground distance units
    pub tgd_unit: Option<WaterDistanceUnit>,
    /// Ground distance since reset
    pub gd: Option<f64>,
    /// Ground distance since reset units
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
