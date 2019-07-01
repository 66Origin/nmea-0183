use crate::parser_utils::parse_string;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Poll a standard message if the current Talker ID is GN
pub struct GNQMessage<'a> {
    /// Message ID of the message to be polled
    pub msg_id: &'a str,
}

pub fn parse_gnq(input: &str) -> IResult<&str, GNQMessage> {
    let (remaining, msg_id) = parse_string(input)?;
    Ok((remaining, GNQMessage { msg_id }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gnq() {
        let input = "RMC";
        let expected = Ok(("", GNQMessage { msg_id: "RMC" }));

        assert_eq!(expected, parse_gnq(input));
    }
}
