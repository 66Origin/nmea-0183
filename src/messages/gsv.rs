use crate::fields::identity::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// GNSS Satellites in View
pub struct GSVMessage {
    /// Number of messages, total number of GSV messages being output
    pub total_msgs: u8,
    /// Number of this message
    pub msg_num: u8,
    /// Number of known satellites in view regarding
    /// both the talker ID and the signalId
    pub satellite_num: u8,
    /// Satellites in view
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
    use crate::fields::distance::Degree;
    use crate::fields::parameter::DBHZ;

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
                        elv: Some(Degree(3.)),
                        az: Some(Degree(111.)),
                        cno: Some(DBHZ(0.)),
                    },
                    SatelliteInView {
                        id: Some(4),
                        elv: Some(Degree(15.)),
                        az: Some(Degree(270.)),
                        cno: Some(DBHZ(0.)),
                    },
                    SatelliteInView {
                        id: Some(6),
                        elv: Some(Degree(1.)),
                        az: Some(Degree(10.)),
                        cno: Some(DBHZ(0.)),
                    },
                    SatelliteInView {
                        id: Some(13),
                        elv: Some(Degree(6.)),
                        az: Some(Degree(292.)),
                        cno: Some(DBHZ(0.)),
                    },
                ],
            },
        ));

        assert_eq!(expected, parse_gsv(input));
    }
}
