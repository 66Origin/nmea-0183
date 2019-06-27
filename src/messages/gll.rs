use crate::fields::cardinality::*;
use crate::fields::parse_u8;
use crate::fields::units::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GLLMessage {
    pub total_msgs: u8,
    pub msg_num: u8,
    pub satellite_num: u8,
    pub satellites: Vec<SatelliteInView>,
}

pub fn parse_gll(input: &str) -> IResult<&str, GLLMessage> {
    let (remaining, (maybe_total_msgs, maybe_msg_num, maybe_satellite_num, satellites)) =
        tuple((parse_u8, parse_u8, parse_u8, parse_satellites_in_view))(input)?;
    match (maybe_total_msgs, maybe_msg_num, maybe_satellite_num) {
        (Some(total_msgs), Some(msg_num), Some(satellite_num)) => Ok((
            remaining,
            GLLMessage {
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
    fn test_parse_gll() {
        let input = "3,1,11,03,03,111,00,04,15,270,00,06,01,010,00,13,06,292,00";
        let expected = Ok((
            "",
            GLLMessage {
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

        assert_eq!(expected, parse_gll(input));
    }
}
