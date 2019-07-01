extern crate chrono;
extern crate nom;

pub use crate::error::Error;

pub mod error;
mod fields;
mod messages;
mod parser_utils;
pub mod sentence;

pub use sentence::*;
pub use fields::cardinality::*;
pub use fields::distance::*;
pub use fields::identity::*;
pub use fields::parameter::*;
pub use fields::speed::*;

/// Parse a sentence According to the NMEA-0183 standard.
///
/// # Examples
///
/// ```
/// # use nmea_0183::parse;
/// # use nmea_0183::Error;
/// # fn main() -> Result<(),  Error<'static>> {
/// // Get a sentence to parse.
/// // According to the specification, an nmea sentence must end with CRLF
/// let raw_nmea = "$GPGGA,092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,*5B\r\n";
/// let parsed_sentence = parse(raw_nmea)?;
///    /*
///    Sentence {
///        sentence_type: Parametric,
///        talker: GPS,
///        message: GGA(GGAMessage {
///            time: Some(09:27:25),
///            lat: Some(Degree(47.1711399)),
///            ns: North,
///            lon: Some(Degree(8.339159)),
///            ew: East,
///            quality: AutonomousGNSSFix,
///            num_sv: Some(8),
///            hdop: Some(1.01),
///            alt: Some(Meter(499.6)),
///            sep: Some(Meter(48.0)),
///            diff_age: None,
///            diff_station: None
///        })
///    }
///    */
/// Ok(())
/// # }
/// ```
pub fn parse(input: &str) -> Result<Sentence, Error> {
    let parse_result = parse_sentence(input)?;
    Ok(parse_result.1)
}

#[cfg(test)]
mod talker_tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let input = "$GPVTG,77.52,T,,M,0.004,N,0.008,K,A*06\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::VTG(messages::VTGMessage {
                cogt: Some(77.52),
                cogt_unit: Some(CourseOverGroundUnit::DegreesTrue),
                cogm: None,
                cogm_unit: Some(CourseOverGroundUnit::DegreesMagnetic),
                sogn: Some(0.004),
                sogn_unit: Some(SpeedOverGroundUnit::Knots),
                sogk: Some(0.008),
                sogk_unit: Some(SpeedOverGroundUnit::KilometersPerHour),
                pos_mode: Fix::AutonomousGNSSFix,
            }),
        };

        assert_eq!(expected_sentence, parse(input).unwrap());
    }

    #[test]
    fn test_parse_missing_crlf() {
        let input = "$GPVTG,77.52,T,,M,0.004,N,0.008,K,A*06";
        let expected_error = Err(Error::ParseError(nom::Err::Error((
            "06",
            nom::error::ErrorKind::TakeUntil,
        ))));
        assert_eq!(expected_error, parse(input));
    }
}
