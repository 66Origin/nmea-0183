use super::structs::*;
use crate::fields::identity::*;
use crate::fields::parameter::SentenceType;
use crate::messages::*;
use nom::bytes::complete::take_until;
use nom::character::complete::crlf;
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn parse_message_type(input: &str) -> IResult<&str, MessageType> {
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

pub fn parse_sentence_type(input: &str) -> IResult<&str, SentenceType> {
    // Array slicing is safe here because nth(0) is Some(_)
    match input.chars().nth(0) {
        Some('$') => Ok((&input[1..], SentenceType::Parametric)),
        Some('!') => Ok((&input[1..], SentenceType::Encapsulation)),
        None => Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete))),
        _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
    }
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
        MessageType::GNS => {
            let (remaining, data) = parse_gns(data_buffer)?;
            (remaining, Message::GNS(data))
        }
        MessageType::GPQ => {
            let (remaining, data) = parse_gpq(data_buffer)?;
            (remaining, Message::GPQ(data))
        }
        MessageType::GRS => {
            let (remaining, data) = parse_grs(data_buffer)?;
            (remaining, Message::GRS(data))
        }
        MessageType::GST => {
            let (remaining, data) = parse_gst(data_buffer)?;
            (remaining, Message::GST(data))
        }
        MessageType::TXT => {
            let (remaining, data) = parse_txt(data_buffer)?;
            (remaining, Message::TXT(data))
        }
        MessageType::VLW => {
            let (remaining, data) = parse_vlw(data_buffer)?;
            (remaining, Message::VLW(data))
        }
        MessageType::VTG => {
            let (remaining, data) = parse_vtg(data_buffer)?;
            (remaining, Message::VTG(data))
        }
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
    computed == checksum
}
