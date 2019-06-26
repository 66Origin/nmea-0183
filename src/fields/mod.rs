use nom::bytes::complete::take_until;
use nom::IResult;

pub mod cardinality;
pub mod sentence_type;
pub mod talker;
pub mod units;

pub fn parse_string(input: &str) -> IResult<&str, &str> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (with_separator, parse_string) = take_until(",")(input)?;
    // Presence of at least one more character has already been checked
    Ok((&with_separator[1..], parse_string))
}

pub fn parse_last_string(input: &str) -> IResult<&str, &str> {
    Ok(("", input))
}

pub fn parse_float(input: &str) -> IResult<&str, Option<f64>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, field) = take_until(",")(input)?;
    // The field is valid, but there is no value
    if field.len() == 0 {
        // Presence of at least one more character has already been checked
        Ok((&remaining[1..], None))
    // The field is a valid float
    } else if let Ok(raw) = field.parse::<f64>() {
        // Presence of at least one more character has already been checked
        Ok((&remaining[1..], Some(raw)))
    // The field is not a valid float
    } else {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    }
}

pub fn parse_u8(input: &str) -> IResult<&str, Option<u8>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, field) = take_until(",")(input)?;
    // The field is valid, but there is no value
    if field.len() == 0 {
        // Presence of at least one more character has already been checked
        Ok((&remaining[1..], None))
    // The field is a valid float
    } else if let Ok(raw) = field.parse::<u8>() {
        // Presence of at least one more character has already been checked
        Ok((&remaining[1..], Some(raw)))
    // The field is not a valid float
    } else {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_float() {
        let lat = "12.34,";
        let expected_value = Some(12.34);
        let expected_remaining_input = "";
        let res = parse_float(lat).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_0_float() {
        let lat = "0.0,";
        let expected_value = Some(0.);
        let expected_remaining_input = "";
        let res = parse_float(lat).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_invalid_float() {
        let lat = "12.34this field is definitely invalid,";
        assert!(parse_float(lat).is_err());
    }

    #[test]
    fn test_parse_empty_float() {
        let lat = ",";
        let expected_value = None;
        let expected_remaining_input = "";
        let res = parse_float(lat).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_string() {
        let input = "foo_bar,";
        let expected = Ok(("", "foo_bar"));
        assert_eq!(expected, parse_string(input));
    }

    #[test]
    fn test_parse_empty_string() {
        let input = ",";
        let expected = Ok(("", ""));
        assert_eq!(expected, parse_string(input));
    }

    #[test]
    fn test_parse_last_string() {
        let input = "foo_bar*";
        let expected = Ok(("", "foo_bar*"));
        assert_eq!(expected, parse_last_string(input));
    }

    #[test]
    fn test_parse_empty_last_string() {
        let input = "";
        let expected = Ok(("", ""));
        assert_eq!(expected, parse_last_string(input));
    }

    #[test]
    fn test_parse_string_sequence() {
        let input = "and one,and two,and three,and four,and five";
        let expected_one = Ok(("and two,and three,and four,and five", "and one"));
        let expected_two = Ok(("and three,and four,and five", "and two"));
        let expected_three = Ok(("and four,and five", "and three"));
        let expected_four = Ok(("and five", "and four"));
        let expected_five = Ok(("", "and five"));
        assert_eq!(expected_one, parse_string(input));
        assert_eq!(expected_two, parse_string(expected_one.unwrap().0));
        assert_eq!(expected_three, parse_string(expected_two.unwrap().0));
        assert_eq!(expected_four, parse_string(expected_three.unwrap().0));
        assert_eq!(expected_five, parse_last_string(expected_four.unwrap().0));
    }
}
