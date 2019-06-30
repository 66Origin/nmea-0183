mod parsers;
mod structs;

pub(crate) use parsers::*;
pub use structs::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_data_invalid() {
        let status = "V,";
        let expected_value = Status::DataInvalid;
        let expected_remaining_input = "";
        let res = parse_status(status).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_status_data_valid() {
        let status = "A,";
        let expected_value = Status::DataValid;
        let expected_remaining_input = "";
        let res = parse_status(status).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_status_missing_comma() {
        let status = "V";
        let expected_value = Status::DataInvalid;
        let expected_remaining_input = "";
        let res = parse_status(status).unwrap();
        assert_eq!(expected_value, res.1);
        assert_eq!(expected_remaining_input, res.0);
    }

    #[test]
    fn test_parse_status_garbage_data() {
        let status = "foo,";
        assert!(parse_status(status).is_err());
    }
}
