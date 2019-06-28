use crate::fields::parse_string;
use crate::fields::parse_u8;
use crate::fields::units::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct TXTMessage<'a> {
    pub num_msg: Option<u8>,
    pub msg_num: Option<u8>,
    pub msg_type: MessageType,
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
                msg_type: MessageType::Notice,
                text: "ANTARIS ATR0620 HW 00000040",
            },
        ));

        assert_eq!(expected, parse_txt(input));
    }
}
