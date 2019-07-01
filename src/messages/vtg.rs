use crate::fields::parameter::*;
use crate::fields::speed::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Course over ground and Ground speed
pub struct VTGMessage {
    /// Course over ground (true)
    pub cogt: Option<f64>,
    /// Course over ground units
    pub cogt_unit: Option<CourseOverGroundUnit>,
    /// Course over ground (magnetic)
    pub cogm: Option<f64>,
    /// Course over ground units
    pub cogm_unit: Option<CourseOverGroundUnit>,
    /// Speed over ground
    pub sogn: Option<f64>,
    /// Speed over ground units
    pub sogn_unit: Option<SpeedOverGroundUnit>,
    /// Speed over ground
    pub sogk: Option<f64>,
    /// Speed over ground units
    pub sogk_unit: Option<SpeedOverGroundUnit>,
    /// Mode Indicator
    pub pos_mode: Fix,
}

pub fn parse_vtg(input: &str) -> IResult<&str, VTGMessage> {
    let (remaining, (cogt, cogt_unit, cogm, cogm_unit, sogn, sogn_unit, sogk, sogk_unit, pos_mode)) =
        tuple((
            parse_float,
            parse_course_over_ground_unit,
            parse_float,
            parse_course_over_ground_unit,
            parse_float,
            parse_speed_over_ground_unit,
            parse_float,
            parse_speed_over_ground_unit,
            parse_pos_mode,
        ))(input)?;
    Ok((
        remaining,
        VTGMessage {
            cogt,
            cogt_unit,
            cogm,
            cogm_unit,
            sogn,
            sogn_unit,
            sogk,
            sogk_unit,
            pos_mode,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vtg() {
        let input = "77.52,T,,M,0.004,N,0.008,K,A";
        let expected = Ok((
            "",
            VTGMessage {
                cogt: Some(77.52),
                cogt_unit: Some(CourseOverGroundUnit::DegreesTrue),
                cogm: None,
                cogm_unit: Some(CourseOverGroundUnit::DegreesMagnetic),
                sogn: Some(0.004),
                sogn_unit: Some(SpeedOverGroundUnit::Knots),
                sogk: Some(0.008),
                sogk_unit: Some(SpeedOverGroundUnit::KilometersPerHour),
                pos_mode: Fix::AutonomousGNSSFix,
            },
        ));

        assert_eq!(expected, parse_vtg(input));
    }
}
