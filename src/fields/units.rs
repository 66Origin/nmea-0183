use crate::fields::parse_float;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Minute(pub f64);
#[derive(Debug, PartialEq)]
pub struct Meter(pub f64);

pub fn parse_minute(input: &str) -> IResult<&str, Option<Minute>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_meter = if let Some(float) = maybe_float {
        Some(Minute(float))
    } else {
        None
    };
    Ok((remaining, maybe_meter))
}

pub fn parse_meter(input: &str) -> IResult<&str, Option<Meter>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_meter = if let Some(float) = maybe_float {
        Some(Meter(float))
    } else {
        None
    };
    Ok((remaining, maybe_meter))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minutes() {
        let lat = "12.34,";
        let expected_value = Some(Minute(12.34));
        let expected_remaining_input = "";
        let res = parse_minute(lat).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_0_minutes() {
        let lat = "0.0,";
        let expected_value = Some(Minute(0.));
        let expected_remaining_input = "";
        let res = parse_minute(lat).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_invalid_minutes() {
        let lat = "12.34this field is definitely invalid,";
        assert!(parse_minute(lat).is_err());
    }

    #[test]
    fn test_parse_meter() {
        let alt = "12.34,";
        let expected_value = Some(Meter(12.34));
        let expected_remaining_input = "";
        let res = parse_meter(alt).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_0_meter() {
        let alt = "0.0,";
        let expected_value = Some(Meter(0.));
        let expected_remaining_input = "";
        let res = parse_meter(alt).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_invalid_meter() {
        let lat = "12.34this field is definitely invalid,";
        assert!(parse_meter(lat).is_err());
    }
}
