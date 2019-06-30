use crate::fields::identity::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GSVMessage {
    pub total_msgs: u8,
    pub msg_num: u8,
    pub satellite_num: u8,
    pub satellites: Vec<SatelliteInView>,
}

pub fn parse_gsv(input: &str) -> IResult<&str, GSVMessage> {
    let (remaining, (maybe_total_msgs, maybe_msg_num, maybe_satellite_num, satellites)) =
        tuple((parse_u8, parse_u8, parse_u8, parse_satellites_in_view))(input)?;
    match (maybe_total_msgs, maybe_msg_num, maybe_satellite_num) {
        (Some(total_msgs), Some(msg_num), Some(satellite_num)) => Ok((
            remaining,
            GSVMessage {
                total_msgs,
                msg_num,
                satellite_num,
                satellites,
            },
        )),
        _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::Not))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gsv() {
        let input = "3,1,11,03,03,111,00,04,15,270,00,06,01,010,00,13,06,292,00";
        let expected = Ok((
            "",
            GSVMessage {
                total_msgs: 3,
                msg_num: 1,
                satellite_num: 11,
                satellites: vec![
                    SatelliteInView {
                        id: Some(3),
                        elv: Some(3),
                        az: Some(111),
                        cno: Some(0),
                    },
                    SatelliteInView {
                        id: Some(4),
                        elv: Some(15),
                        az: Some(270),
                        cno: Some(0),
                    },
                    SatelliteInView {
                        id: Some(6),
                        elv: Some(1),
                        az: Some(10),
                        cno: Some(0),
                    },
                    SatelliteInView {
                        id: Some(13),
                        elv: Some(6),
                        az: Some(292),
                        cno: Some(0),
                    },
                ],
            },
        ));

        assert_eq!(expected, parse_gsv(input));
    }
}
