use crate::parser_utils::parse_string;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GPQMessage<'a> {
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
