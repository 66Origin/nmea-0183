use crate::fields::sentence_type::parse_sentence_type;
use crate::fields::sentence_type::SentenceType;
use crate::fields::talker::parse_talker;
use crate::fields::talker::Talker;
use crate::messages::dtm::*;
use crate::messages::gbq::*;
use crate::messages::gga::*;
use crate::messages::gll::*;
use crate::messages::glq::*;
use crate::messages::gnq::*;
use crate::messages::gsa::*;
use crate::messages::gsv::*;
use crate::messages::rmc::*;
use crate::messages::zda::*;
use crate::messages::gbs::*;
use nom::bytes::complete::take_until;
use nom::character::complete::crlf;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Message<'a> {
    DTM(DTMMessage<'a>),
    GBQ(GBQMessage<'a>),
    GBS(GBSMessage),
    GGA(GGAMessage),
    GLL(GLLMessage),
    GLQ(GLQMessage<'a>),
    GNQ(GNQMessage<'a>),
    GNS,
    GPQ,
    GRS,
    GSA(GSAMessage),
    GST,
    GSV(GSVMessage),
    RMC(RMCMessage),
    TXT,
    VLW,
    VTG,
    ZDA(ZDAMessage),
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    DTM,
    GBQ,
    GBS,
    GGA,
    GLL,
    GLQ,
    GNQ,
    GNS,
    GPQ,
    GRS,
    GSA,
    GST,
    GSV,
    RMC,
    TXT,
    VLW,
    VTG,
    ZDA,
}

fn parse_message_type(input: &str) -> IResult<&str, MessageType> {
    if input.len() < 4 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (maybe_message_type, remaining) = input.split_at(4);
    match maybe_message_type {
        "DTM," => Ok((remaining, MessageType::DTM)),
        "GBQ," => Ok((remaining, MessageType::GBQ)),
        "GBS," => Ok((remaining, MessageType::GBS)),
        "GGA," => Ok((remaining, MessageType::GGA)),
        "GLL," => Ok((remaining, MessageType::GLL)),
        "GLQ," => Ok((remaining, MessageType::GLQ)),
        "GNQ," => Ok((remaining, MessageType::GNQ)),
        "GNS," => Ok((remaining, MessageType::GNS)),
        "GPQ," => Ok((remaining, MessageType::GPQ)),
        "GRS," => Ok((remaining, MessageType::GRS)),
        "GSA," => Ok((remaining, MessageType::GSA)),
        "GST," => Ok((remaining, MessageType::GST)),
        "GSV," => Ok((remaining, MessageType::GSV)),
        "RMC," => Ok((remaining, MessageType::RMC)),
        "TXT," => Ok((remaining, MessageType::TXT)),
        "VLW," => Ok((remaining, MessageType::VLW)),
        "VTG," => Ok((remaining, MessageType::VTG)),
        "ZDA," => Ok((remaining, MessageType::ZDA)),
        _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
    }
}

#[derive(Debug, PartialEq)]
pub struct Sentence<'a> {
    sentence_type: SentenceType,
    talker: Talker,
    message: Message<'a>,
}

pub fn parse_sentence(input: &str) -> IResult<&str, Sentence> {
    let (remaining, sentence_type) = parse_sentence_type(input)?;
    let (data_buffer, (talker, message_type)) = get_headers_if_sentence_valid(remaining)?;

    let (remaining_data, message) = match message_type {
        MessageType::DTM => {
            let (remaining, data) = parse_dtm(data_buffer)?;
            (remaining, Message::DTM(data))
        }
        MessageType::GBQ => {
            let (remaining, data) = parse_gbq(data_buffer)?;
            (remaining, Message::GBQ(data))
        }
        MessageType::GGA => {
            let (remaining, data) = parse_gga(data_buffer)?;
            (remaining, Message::GGA(data))
        }
        MessageType::GSA => {
            let (remaining, data) = parse_gsa(data_buffer)?;
            (remaining, Message::GSA(data))
        }
        MessageType::GSV => {
            let (remaining, data) = parse_gsv(data_buffer)?;
            (remaining, Message::GSV(data))
        }
        MessageType::GLL => {
            let (remaining, data) = parse_gll(data_buffer)?;
            (remaining, Message::GLL(data))
        }
        MessageType::ZDA => {
            let (remaining, data) = parse_zda(data_buffer)?;
            (remaining, Message::ZDA(data))
        }
        MessageType::RMC => {
            let (remaining, data) = parse_rmc(data_buffer)?;
            (remaining, Message::RMC(data))
        }
        MessageType::GLQ => {
            let (remaining, data) = parse_glq(data_buffer)?;
            (remaining, Message::GLQ(data))
        }
        MessageType::GNQ => {
            let (remaining, data) = parse_gnq(data_buffer)?;
            (remaining, Message::GNQ(data))
        }
        MessageType::GBS => {
            let (remaining, data) = parse_gbs(data_buffer)?;
            (remaining, Message::GBS(data))
        }
        _ => unimplemented!(),
    };

    if remaining_data.len() == 0 {
        Ok((
            remaining_data,
            Sentence {
                sentence_type,
                talker,
                message,
            },
        ))
    } else {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::NonEmpty)));
    }
}

fn get_headers_if_sentence_valid(input: &str) -> IResult<&str, (Talker, MessageType)> {
    let (after_data, data) = take_until("*")(input)?;
    // Index subscription is safe because take_until does not consume the pattern
    let (after_checksum, checksum) = parse_checksum(&after_data[1..])?;
    if !sentence_is_valid(data, checksum) {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Verify)));
    }
    if crlf(after_checksum)?.0.len() != 0 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::NonEmpty)));
    }
    Ok(tuple((parse_talker, parse_message_type))(data)?)
}

fn parse_checksum(input: &str) -> IResult<&str, u8> {
    let (after_cs, maybe_cs) = take_until("\r")(input)?;
    if let Ok(cs) = decode_cs(maybe_cs) {
        Ok((after_cs, cs))
    } else {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)))
    }
}

fn decode_cs(s: &str) -> Result<u8, nom::Err<(&str, nom::error::ErrorKind)>> {
    // The checksum is supposed to be 2 characters wide
    if s.chars().nth(1).is_none() {
        return Err(nom::Err::Failure((s, nom::error::ErrorKind::Complete)));
    } else {
        u8::from_str_radix(&s[0..2], 16)
            .map_err(|_| nom::Err::Failure((s, nom::error::ErrorKind::Digit)))
    }
}

fn sentence_is_valid(data: &str, checksum: u8) -> bool {
    let computed = data.chars().fold(0, |sum, c| sum ^ c as u8);
    println!("{:X} ", computed);
    println!("{:X} ", checksum);
    computed == checksum
}

#[cfg(test)]
mod talker_tests {
    use super::*;
    use crate::fields::cardinality::{EastWest, NorthSouth};
    use crate::fields::units::*;
    use chrono::naive::{NaiveDate, NaiveTime};

    #[test]
    fn test_parse_dtm_0_lat_lon_alt() {
        let input = "$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DTMMessage {
                datum: "W84",
                sub_datum: "",
                lat: Some(Minute(0.)),
                ns: NorthSouth::North,
                lon: Some(Minute(0.)),
                ew: EastWest::East,
                alt: Some(Meter(0.)),
                ref_datum: "W84",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_dtm_lat_lon_alt() {
        let input = "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DTMMessage {
                datum: "999",
                sub_datum: "",
                lat: Some(Minute(0.08)),
                ns: NorthSouth::North,
                lon: Some(Minute(0.07)),
                ew: EastWest::East,
                alt: Some(Meter(-47.7)),
                ref_datum: "W84",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_dtm_no_lat_lon_alt() {
        let input = "$GPDTM,999,,,N,,E,,W84*23\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DTMMessage {
                datum: "999",
                sub_datum: "",
                lat: None,
                ns: NorthSouth::North,
                lon: None,
                ew: EastWest::East,
                alt: None,
                ref_datum: "W84",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gbq() {
        let input = "$UPGBQ,RMC*21\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::MicroprocessorController,
            message: Message::GBQ(GBQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gga() {
        let input = "$GPGGA,092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,*5B\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GGA(GGAMessage {
                time: Some(NaiveTime::from_hms_milli(09, 27, 25, 00)),
                lat: Some(Degree(47.1711399)),
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391590)),
                ew: EastWest::East,
                quality: Fix::AutonomousGNSSFix,
                num_sv: Some(8),
                hdop: Some(1.01),
                alt: Some(Meter(499.6)),
                sep: Some(Meter(48.)),
                diff_age: None,
                diff_station: None,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gsa() {
        let input = "$GNGSA,A,3,80,71,73,79,69,,,,,,,,1.83,1.09,1.47*17\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPSGLONASS,
            message: Message::GSA(GSAMessage {
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
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gsv() {
        let input = "$GPGSV,3,1,11,03,03,111,00,04,15,270,00,06,01,010,00,13,06,292,00*74\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GSV(GSVMessage {
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
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gll() {
        let input = "$GPGLL,4717.11364,N,00833.91565,E,092321.00,A,A*60\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GLL(GLLMessage {
                lat: Some(Degree(47.171136399999995)), // floats ¯\_(ツ)_/¯
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391565)),
                ew: EastWest::East,
                time: Some(NaiveTime::from_hms(9, 23, 21)),
                status: Status::DataValid,
                pos_mode: Fix::AutonomousGNSSFix,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_zda() {
        let input = "$GPZDA,082710.00,16,09,2002,00,00*64\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::ZDA(ZDAMessage {
                time: Some(NaiveTime::from_hms(8, 27, 10)),
                day: Some(16),
                month: Some(09),
                year: Some(2002),
                ltzh: Some(0),
                ltzn: Some(0),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_rmc() {
        let input = "$GPRMC,083559.00,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A,V*2D\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::RMC(RMCMessage {
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
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_glq() {
        let input = "$UPGLQ,RMC*2F\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::MicroprocessorController,
            message: Message::GLQ(GLQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gnq() {
        let input = "$UPGNQ,RMC*2D\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::MicroprocessorController,
            message: Message::GNQ(GNQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gbs() {
        let input = "$GPGBS,235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0*5A\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GBS(GBSMessage {
                time: Some(NaiveTime::from_hms(23, 54, 58)),
                lat_err: Some(Meter(1.4)),
                lon_err: Some(Meter(1.3)),
                alt_err: Some(Meter(3.1)),
                sat_prn: Some(3),
                prob: None,
                res: Some(-21.4),
                std_dev: Some(3.8),
                system_id: Some(1),
                signal_id: Some(0),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }
}
