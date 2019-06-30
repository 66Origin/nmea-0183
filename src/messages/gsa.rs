use crate::fields::distance::*;
use crate::fields::identity::*;
use crate::fields::parameter::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GSAMessage {
    pub op_mode: OperationMode,
    pub nav_mode: NavigationMode,
    pub sattelite_ids: [Option<u8>; 12],
    pub pdop: Option<Meter>,
    pub hdop: Option<Meter>,
    pub vdop: Option<Meter>,
}

pub fn parse_gsa(input: &str) -> IResult<&str, GSAMessage> {
    let (remaining, (op_mode, nav_mode, sattelite_ids, pdop, hdop, vdop)) = tuple((
        parse_operation_mode,
        parse_navigation_mode,
        parse_satellite_ids,
        parse_meter,
        parse_meter,
        parse_last_meter,
    ))(input)?;
    Ok((
        remaining,
        GSAMessage {
            op_mode,
            nav_mode,
            sattelite_ids,
            pdop,
            hdop,
            vdop,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gsa() {
        let input = "A,3,80,71,73,79,69,,,,,,,,1.83,1.09,1.47";
        let expected = Ok((
            "",
            GSAMessage {
                op_mode: OperationMode::Automatic,
                nav_mode: NavigationMode::Fix3D,
                sattelite_ids: [
                    Some(80),
                    Some(71),
                    Some(73),
                    Some(79),
                    Some(69),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                pdop: Some(Meter(1.83)),
                hdop: Some(Meter(1.09)),
                vdop: Some(Meter(1.47)),
            },
        ));

        assert_eq!(expected, parse_gsa(input));
    }
}
