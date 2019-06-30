use super::structs::*;
use crate::parser_utils::*;
use nom::IResult;

pub fn parse_course_over_ground_unit(input: &str) -> IResult<&str, Option<CourseOverGroundUnit>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('T') => (&input[1..], Some(CourseOverGroundUnit::DegreesTrue)),
        Some('M') => (&input[1..], Some(CourseOverGroundUnit::DegreesMagnetic)),
        Some(',') => (&input[1..], None),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_speed_over_ground_unit(input: &str) -> IResult<&str, Option<SpeedOverGroundUnit>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('N') => (&input[1..], Some(SpeedOverGroundUnit::Knots)),
        Some('K') => (&input[1..], Some(SpeedOverGroundUnit::KilometersPerHour)),
        Some(',') => (&input[1..], None),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_water_distance_unit(input: &str) -> IResult<&str, Option<WaterDistanceUnit>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.chars().nth(0) {
        // Index subscription is safe since input has at least 1 char
        Some('N') => (&input[1..], Some(WaterDistanceUnit::NauticalMile)),
        Some(',') => (&input[1..], None),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_knot(input: &str) -> IResult<&str, Option<Knot>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_knot = if let Some(float) = maybe_float {
        Some(Knot(float))
    } else {
        None
    };
    Ok((remaining, maybe_knot))
}
