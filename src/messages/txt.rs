use crate::fields::parameter::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Text Transmission
pub struct TXTMessage<'a> {
    /// Total number of messages in thistransmission
    pub num_msg: Option<u8>,
    /// Message number in this transmission
    pub msg_num: Option<u8>,
    /// Text identifier
    pub msg_type: MessageLevel,
    /// The payload
    pub text: &'a str,
}

pub fn parse_txt(input: &str) -> IResult<&str, TXTMessage> {
    let (remaining, (num_msg, msg_num, msg_type, text)) =
        tuple((parse_u8, parse_u8, parse_message_type, parse_string))(input)?;
    Ok((
        remaining,
        TXTMessage {
            num_msg,
            msg_num,
            msg_type,
            text,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_txt() {
        let input = "01,01,02,ANTARIS ATR0620 HW 00000040";
        let expected = Ok((
            "",
            TXTMessage {
                num_msg: Some(01),
                msg_num: Some(01),
                msg_type: MessageLevel::Notice,
                text: "ANTARIS ATR0620 HW 00000040",
            },
        ));

        assert_eq!(expected, parse_txt(input));
    }
}
