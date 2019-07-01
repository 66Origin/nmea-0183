use crate::fields::cardinality::*;
use crate::fields::distance::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Datum Reference
///
/// This message gives the difference between the current datum and the referencedatum.
/// The current datum defaults to WGS84.
/// The reference datum cannot be changed and is always set to WGS84.
pub struct DTMMessage<'a> {
    /// Local datum code
    pub datum: &'a str,
    /// A null field
    pub sub_datum: &'a str,
    /// Offset in Latitude
    pub lat: Option<Minute>,
    /// North/South indicator
    pub ns: NorthSouth,
    /// Offset in Longitude
    pub lon: Option<Minute>,
    /// East/West indicator
    pub ew: EastWest,
    /// Offset in altitude
    pub alt: Option<Meter>,
    /// Reference datum code
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
        parse_string,
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
