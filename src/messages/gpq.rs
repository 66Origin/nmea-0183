use crate::parser_utils::parse_string;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Poll a standard message if the current Talker ID is GP
pub struct GPQMessage<'a> {
    /// Message ID of the message to be polled
    pub msg_id: &'a str,
}

pub fn parse_gpq(input: &str) -> IResult<&str, GPQMessage> {
    let (remaining, msg_id) = parse_string(input)?;
    Ok((remaining, GPQMessage { msg_id }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gpq() {
        let input = "RMC";
        let expected = Ok(("", GPQMessage { msg_id: "RMC" }));

        assert_eq!(expected, parse_gpq(input));
    }
}
