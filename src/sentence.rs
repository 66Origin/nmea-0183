use crate::fields::sentence_type::parse_sentence_type;
use crate::fields::sentence_type::SentenceType;
use crate::fields::talker::parse_talker;
use crate::fields::talker::Talker;
use crate::messages::dtm::parse_dtm;
use crate::messages::dtm::DTMMessage;
use crate::messages::gbq::parse_gbq;
use crate::messages::gbq::GBQMessage;
use crate::messages::gga::parse_gga;
use crate::messages::gga::GGAMessage;
use nom::bytes::complete::take_until;
use nom::character::complete::crlf;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Message<'a> {
    DTM(DTMMessage<'a>),
    GBQ(GBQMessage<'a>),
    GBS,
    GGA(GGAMessage),
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
    use crate::fields::units::Degree;
    use crate::fields::units::Fix;
    use crate::fields::units::Meter;
    use crate::fields::units::Minute;
    use chrono::naive::NaiveTime;

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
    fn test_parse_dtm2() {
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
}
