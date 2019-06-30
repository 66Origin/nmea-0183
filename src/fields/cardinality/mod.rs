mod parsers;
mod structs;

pub(crate) use parsers::*;
pub use structs::*;

#[cfg(test)]
mod cardinality_tests {
    use super::*;

    #[test]
    fn test_parse_north_indicator() {
        let input = "N,";
        let expected_output = Ok(("", NorthSouth::North));
        assert_eq!(expected_output, parse_north_south_indicator(input));
    }

    #[test]
    fn test_parse_south_indicator() {
        let input = "S,";
        let expected_output = Ok(("", NorthSouth::South));
        assert_eq!(expected_output, parse_north_south_indicator(input));
    }

    #[test]
    fn test_parse_empty_indicator() {
        let input = ",";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        assert_eq!(expected_output, parse_north_south_indicator(input));
    }

    #[test]
    fn test_parse_east_indicator() {
        let input = "E,";
        let expected_output = Ok(("", EastWest::East));
        assert_eq!(expected_output, parse_east_west_indicator(input));
    }

    #[test]
    fn test_parse_west_indicator() {
        let input = "W,";
        let expected_output = Ok(("", EastWest::West));
        assert_eq!(expected_output, parse_east_west_indicator(input));
    }

    #[test]
    fn test_parse_empty_indicators() {
        let input = ",";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        let expected_output2 = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        assert_eq!(expected_output, parse_east_west_indicator(input));
        assert_eq!(expected_output2, parse_north_south_indicator(input));
    }

    #[test]
    fn test_parse_wrong_indicators() {
        let input = "K,";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        let expected_output2 = Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));

        assert_eq!(expected_output, parse_east_west_indicator(input));
        assert_eq!(expected_output2, parse_north_south_indicator(input));
    }
}
