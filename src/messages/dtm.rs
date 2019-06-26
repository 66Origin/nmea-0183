use crate::fields::cardinality::EastWest;
use crate::fields::cardinality::NorthSouth;
use crate::fields::parse_last_string;
use crate::fields::parse_string;
use crate::fields::units::parse_meter;
use crate::fields::units::parse_minute;
use crate::fields::units::Meter;
use crate::fields::{
    cardinality::{parse_east_west_indicator, parse_north_south_indicator},
    units::Minute,
};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct DTMMessage<'a> {
    pub datum: &'a str,
    pub sub_datum: &'a str,
    pub lat: Option<Minute>,
    pub ns: NorthSouth,
    pub lon: Option<Minute>,
    pub ew: EastWest,
    pub alt: Option<Meter>,
    pub ref_datum: &'a str,
}

pub fn parse_dtm(input: &str) -> IResult<&str, DTMMessage> {
    let (remaining, datum_ref) = tuple((
        parse_string,
        parse_string,
        parse_minute,
        parse_north_south_indicator,
        parse_minute,
        parse_east_west_indicator,
        parse_meter,
        parse_last_string,
    ))(input)?;
    Ok((
        remaining,
        DTMMessage {
            datum: datum_ref.0,
            sub_datum: datum_ref.1,
            lat: datum_ref.2,
            ns: datum_ref.3,
            lon: datum_ref.4,
            ew: datum_ref.5,
            alt: datum_ref.6,
            ref_datum: datum_ref.7,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::cardinality::{EastWest, NorthSouth};

    #[test]
    fn test_parse_dtm() {
        let input = "W84,,0.0,N,0.0,E,0.0,W84";
        let expected = Ok((
            "",
            DTMMessage {
                datum: "W84",
                sub_datum: "",
                lat: Some(Minute(0.)),
                ns: NorthSouth::North,
                lon: Some(Minute(0.)),
                ew: EastWest::East,
                alt: Some(Meter(0.)),
                ref_datum: "W84",
            },
        ));

        assert_eq!(expected, parse_dtm(input));
    }
}
