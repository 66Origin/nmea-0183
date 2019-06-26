use crate::fields::sentence_type::parse_sentence_type;
use crate::fields::sentence_type::SentenceType;
use crate::fields::talker::parse_talker;
use crate::fields::talker::Talker;
use crate::messages::datum_reference::parse_datum_reference;
use crate::messages::datum_reference::DatumReference;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Message<'a> {
    DTM(DatumReference<'a>),
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

fn parse_sentence(input: &str) -> IResult<&str, Sentence> {
    let (remaining, sentence_ref) =
        tuple((parse_sentence_type, parse_talker, parse_message_type))(input)?;
    //TODO: checksum + CRLF here
    let after_crlf = "";
    let (_, message) = match sentence_ref.2 {
        MessageType::DTM => {
            let (remaining, data) = parse_datum_reference(remaining)?;
            (remaining, Message::DTM(data))
        }
        _ => unimplemented!(),
    };

    Ok((
        after_crlf,
        Sentence {
            sentence_type: sentence_ref.0,
            talker: sentence_ref.1,
            message,
        },
    ))
}

#[cfg(test)]
mod talker_tests {
    use super::*;
    use crate::fields::cardinality::{EastWest, NorthSouth};
    use crate::fields::units::Meter;
    use crate::fields::units::Minute;

    #[test]
    fn test_parse_dtm() {
        let input = "$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DatumReference {
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
        let input = "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1C";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DatumReference {
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
        let input = "$GPDTM,999,,,N,,E,,W84*1C";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DatumReference {
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
}
