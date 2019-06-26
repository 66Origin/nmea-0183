use crate::fields::parse_last_string;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GBQMessage<'a> {
    pub msg_id: &'a str,
}

pub fn parse_gbq(input: &str) -> IResult<&str, GBQMessage> {
    let (remaining, msg_id) = parse_last_string(input)?;
    Ok((remaining, GBQMessage { msg_id }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gbq() {
        let input = "RMC";
        let expected = Ok(("", GBQMessage { msg_id: "RMC" }));

        assert_eq!(expected, parse_gbq(input));
    }
}
