use super::structs::*;
use crate::parser_utils::*;
use nom::bytes::complete::take_until;
use nom::IResult;

pub fn parse_computation_method(input: &str) -> IResult<&str, Option<ComputationMethod>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('0') => (&input[1..], Some(ComputationMethod::InGGA)),
        Some('1') => (&input[1..], Some(ComputationMethod::AfterGGA)),
        Some(',') => (&input[1..], None),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_navigational_status(input: &str) -> IResult<&str, NavigationalStatus> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('S') => (&input[1..], NavigationalStatus::Safe),
        Some('C') => (&input[1..], NavigationalStatus::Caution),
        Some('U') => (&input[1..], NavigationalStatus::Unsafe),
        Some('V') => (&input[1..], NavigationalStatus::NotValid),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_navigation_mode(input: &str) -> IResult<&str, NavigationMode> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('1') => (&input[1..], NavigationMode::FixNo),
        Some('2') => (&input[1..], NavigationMode::Fix2D),
        Some('3') => (&input[1..], NavigationMode::Fix3D),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_operation_mode(input: &str) -> IResult<&str, OperationMode> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('M') => (&input[1..], OperationMode::Manual),
        Some('A') => (&input[1..], OperationMode::Automatic),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_status(input: &str) -> IResult<&str, Status> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('V') => (&input[1..], Status::DataInvalid),
        Some('A') => (&input[1..], Status::DataValid),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_quality(input: &str) -> IResult<&str, Fix> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }

    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 item
        Some('0') => (&input[2..], Fix::NoFix),
        Some('1') => (&input[2..], Fix::AutonomousGNSSFix),
        Some('2') => (&input[2..], Fix::DifferentialGNSSFix),
        Some('4') => (&input[2..], Fix::RTKFixed),
        Some('5') => (&input[2..], Fix::RTKFloat),
        Some('6') => (&input[2..], Fix::EstimatedOrDeadReckoningFix),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };

    remove_separator_if_next(',', remaining, result)
}

pub fn parse_pos_mode_vec(input: &str) -> IResult<&str, FixList> {
    let (remaining, mut pos_modes_str) = take_until(",")(input)?;
    // Should actually be pos_modes_str,len() -1
    // But i dont want to deal with negative values
    let mut pos_modes = FixList::new();

    while pos_modes_str.len() > 0 {
        let res = parse_pos_mode(pos_modes_str)?;
        pos_modes_str = res.0;
        pos_modes.push(res.1);
    }

    remove_separator_if_next(',', remaining, pos_modes)
}

pub fn parse_pos_mode(input: &str) -> IResult<&str, Fix> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 item
        Some('N') => (&input[1..], Fix::NoFix),
        Some('A') => (&input[1..], Fix::AutonomousGNSSFix),
        Some('D') => (&input[1..], Fix::DifferentialGNSSFix),
        Some('R') => (&input[1..], Fix::RTKFixed),
        Some('F') => (&input[1..], Fix::RTKFloat),
        Some('E') => (&input[1..], Fix::EstimatedOrDeadReckoningFix),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_message_type(input: &str) -> IResult<&str, MessageLevel> {
    let (remaining, type_str) = take_until(",")(input)?;
    let result = match type_str {
        "00" => MessageLevel::Error,
        "01" => MessageLevel::Warning,
        "02" => MessageLevel::Notice,
        "07" => MessageLevel::User,
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    // Index subscription is safe since input has at least the comma
    Ok((&remaining[1..], result))
}

pub fn parse_dilution_of_precision(input: &str) -> IResult<&str, Option<f64>> {
    parse_float(input)
}

pub fn parse_dbhz(input: &str) -> IResult<&str, Option<DBHZ>> {
    let (remaining, sec) = parse_float(input)?;
    if let Some(s) = sec {
        Ok((remaining, Some(DBHZ(s))))
    } else {
        Ok((remaining, None))
    }
}
