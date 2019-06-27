use crate::fields::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Minute(pub f64);

#[derive(Debug, PartialEq)]
pub struct Degree(pub f64);

#[derive(Debug, PartialEq)]
pub struct Meter(pub f64);

#[derive(Debug, PartialEq)]
pub struct Second(pub f64);

#[derive(Debug, PartialEq)]
pub enum Fix {
    NoFix,
    AutonomousGNSSFix,
    DifferentialGNSSFix,
    RTKFixed,
    RTKFloat,
    EstimatedOrDeadReckoningFix,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    DataInvalid,
    DataValid,
}

#[derive(Debug, PartialEq)]
pub enum OperationMode {
    Manual,
    Automatic,
}

#[derive(Debug, PartialEq)]
pub enum NavigationMode {
    FixNo,
    Fix2D,
    Fix3D,
}

#[derive(Debug, PartialEq)]
pub struct SatelliteInView {
    pub id: Option<u8>,
    pub elv: Option<u8>,
    pub az: Option<u16>,
    pub cno: Option<u8>,
}

pub fn parse_satellites_in_view(input: &str) -> IResult<&str, Vec<SatelliteInView>> {
    let mut remaining = input;
    let mut satellites = Vec::new();
    while remaining.len() != 0 {
        let sv = parse_satellite_in_view(remaining)?;
        remaining = sv.0;
        satellites.push(sv.1);
    }
    Ok((remaining, satellites))
}

fn parse_satellite_in_view(input: &str) -> IResult<&str, SatelliteInView> {
    let (remaining, (id, elv, az, cno)) = tuple((parse_u8, parse_u8, parse_u16, parse_u8))(input)?;
    Ok((remaining, SatelliteInView { id, elv, az, cno }))
}

pub fn parse_navigation_mode(input: &str) -> IResult<&str, NavigationMode> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('1') => Ok((&input[2..], NavigationMode::FixNo)),
            Some('2') => Ok((&input[2..], NavigationMode::Fix2D)),
            Some('3') => Ok((&input[2..], NavigationMode::Fix3D)),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_satellite_ids(input: &str) -> IResult<&str, [Option<u8>; 12]> {
    let mut remaining = input;
    let mut ids = [None; 12];
    for i in 0..12 {
        let parsed = parse_satellite_id(remaining)?;
        remaining = parsed.0;
        ids[i] = parsed.1;
    }

    Ok((remaining, ids))
}

fn parse_satellite_id(input: &str) -> IResult<&str, Option<u8>> {
    parse_u8(input).or_else(|err| match err {
        nom::Err::Failure((input, nom::error::ErrorKind::Complete)) => Ok((input, None)),
        _ => Err(err),
    })
}

pub fn parse_operation_mode(input: &str) -> IResult<&str, OperationMode> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('M') => Ok((&input[2..], OperationMode::Manual)),
            Some('A') => Ok((&input[2..], OperationMode::Automatic)),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_status(input: &str) -> IResult<&str, Status> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('V') => Ok((&input[2..], Status::DataInvalid)),
            Some('A') => Ok((&input[2..], Status::DataValid)),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_quality(input: &str) -> IResult<&str, Fix> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('0') => Ok((&input[2..], Fix::NoFix)),
            Some('1') => Ok((&input[2..], Fix::AutonomousGNSSFix)),
            Some('2') => Ok((&input[2..], Fix::DifferentialGNSSFix)),
            Some('4') => Ok((&input[2..], Fix::RTKFixed)),
            Some('5') => Ok((&input[2..], Fix::RTKFloat)),
            Some('6') => Ok((&input[2..], Fix::EstimatedOrDeadReckoningFix)),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_pos_mode(input: &str) -> IResult<&str, Fix> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('N') => Ok((&input[2..], Fix::NoFix)),
            Some('A') => Ok((&input[2..], Fix::AutonomousGNSSFix)),
            Some('D') => Ok((&input[2..], Fix::DifferentialGNSSFix)),
            Some('R') => Ok((&input[2..], Fix::RTKFixed)),
            Some('F') => Ok((&input[2..], Fix::RTKFloat)),
            Some('E') => Ok((&input[2..], Fix::EstimatedOrDeadReckoningFix)),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_minute(input: &str) -> IResult<&str, Option<Minute>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_minute = if let Some(float) = maybe_float {
        Some(Minute(float))
    } else {
        None
    };
    Ok((remaining, maybe_minute))
}

pub fn parse_degree(input: &str) -> IResult<&str, Option<Degree>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_degree = if let Some(float) = maybe_float {
        Some(Degree(float / 100.)) // 4717.11399 is actually
    } else {
        None
    };
    Ok((remaining, maybe_degree))
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

pub fn parse_last_meter(input: &str) -> IResult<&str, Option<Meter>> {
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

pub fn ensure_meter(input: &str) -> IResult<&str, ()> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('M') => Ok((&input[2..], ())),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_num_satellites(input: &str) -> IResult<&str, Option<u8>> {
    parse_u8(input)
}

pub fn parse_station(input: &str) -> IResult<&str, Option<u8>> {
    parse_u8(input).or_else(|err| match err {
        nom::Err::Failure((input, nom::error::ErrorKind::Complete)) => Ok((input, None)),
        _ => Err(err),
    })
}

pub fn parse_second(input: &str) -> IResult<&str, Option<Second>> {
    let (remaining, sec) = parse_float(input)?;
    if let Some(s) = sec {
        Ok((remaining, Some(Second(s))))
    } else {
        Ok((remaining, None))
    }
}

pub fn parse_dilution_of_precision(input: &str) -> IResult<&str, Option<f64>> {
    parse_float(input)
}

pub fn parse_system(input: &str) -> IResult<&str, u8> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    Ok((input, 0))
}

pub fn parse_signal(input: &str) -> IResult<&str, u8> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    Ok((input, 0))
}

// 235503.00
// 125027
pub fn parse_time(input: &str) -> IResult<&str, Option<NaiveTime>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, time_str) = parse_string(input)?;

    let splitted: Vec<&str> = time_str.split('.').collect();

    let maybe_time = match (splitted.get(0), splitted.get(1)) {
        (Some(hms), Some(milis)) => {
            if let Ok(raw_hms) = str::parse::<u32>(hms) {
                let hours = raw_hms / 10_000;
                let minutes = (raw_hms - hours * 10_000) / 100;
                let seconds = raw_hms - hours * 10_000 - minutes * 100;
                if let Ok(raw_milis) = str::parse::<u32>(milis) {
                    NaiveTime::from_hms_milli_opt(hours, minutes, seconds, raw_milis)
                } else {
                    return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
                }
            } else {
                return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
            }
        }
        (Some(hms), None) => {
            if let Ok(raw_hms) = str::parse::<u32>(hms) {
                let hours = raw_hms / 100;
                let minutes = (raw_hms - hours) / 100;
                let seconds = raw_hms - hours - minutes;
                NaiveTime::from_hms_opt(hours, minutes, seconds)
            } else {
                return Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)));
            }
        }
        _ => None,
    };

    Ok((remaining, maybe_time))
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
        assert!(parse_status(status).is_err());
    }

    #[test]
    fn test_parse_status_garbage_data() {
        let status = "foo,";
        assert!(parse_status(status).is_err());
    }
}
