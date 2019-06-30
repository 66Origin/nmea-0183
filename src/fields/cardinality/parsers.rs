use super::structs::*;
use crate::parser_utils::remove_separator_if_next;
use nom::{IResult, InputTake};

pub fn parse_north_south_indicator(input: &str) -> IResult<&str, NorthSouth> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.take(1) {
        "N" => (&input[1..], NorthSouth::North),
        "S" => (&input[1..], NorthSouth::South),
        "," => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        }
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_maybe_north_south_indicator(input: &str) -> IResult<&str, Option<NorthSouth>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.take(1) {
        "N" => (&input[1..], Some(NorthSouth::North)),
        "S" => (&input[1..], Some(NorthSouth::South)),
        "," => (&input[1..], None),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_maybe_east_west_indicator(input: &str) -> IResult<&str, Option<EastWest>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, result) = match input.take(1) {
        "E" => (&input[1..], Some(EastWest::East)),
        "W" => (&input[1..], Some(EastWest::West)),
        "," => (&input[1..], None),
        _ => {
            return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        }
    };
    remove_separator_if_next(',', remaining, result)
}

pub fn parse_east_west_indicator(input: &str) -> IResult<&str, EastWest> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    match input.take(1) {
        "E" => Ok((&input[2..], EastWest::East)),
        "W" => Ok((&input[2..], EastWest::West)),
        _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
    }
}
