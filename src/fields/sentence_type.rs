use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum SentenceType {
    Parametric,
    Encapsulation,
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

#[cfg(test)]
mod sentence_type_tests {
    use super::*;

    #[test]
    fn test_parse_parametric_parse_sentence_type() {
        let input = "$foo bar";
        let expected_output = Ok(("foo bar", SentenceType::Parametric));
        assert_eq!(expected_output, parse_sentence_type(input));
    }

    #[test]
    fn test_parse_encapsulation_parse_sentence_type() {
        let input = "!foo bar";
        let expected_output = Ok(("foo bar", SentenceType::Encapsulation));
        assert_eq!(expected_output, parse_sentence_type(input));
    }

    #[test]
    fn test_parse_encapsulation_invalid_type() {
        let input = "*foo bar";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        assert_eq!(expected_output, parse_sentence_type(input));
    }

    #[test]
    fn test_parse_encapsulation_empty_input() {
        let input = "";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        assert_eq!(expected_output, parse_sentence_type(input));
    }
}
