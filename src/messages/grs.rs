use crate::fields::cardinality::*;
use crate::fields::units::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GRSMessage {
    pub time: Option<NaiveTime>,
    pub mode: Option<ComputationMethod>,
    pub residuals: [Option<Meter>; 12],
    pub system_id: Option<u8>,
    pub signal_id: Option<u8>,
}

pub fn parse_grs(input: &str) -> IResult<&str, GRSMessage> {
    let (remaining, (time, mode, residuals, system_id, signal_id)) = tuple((
        parse_time,
        parse_computation_method,
        parse_residuals,
        parse_system,
        parse_signal,
    ))(input)?;
    Ok((
        remaining,
        GRSMessage {
            time,
            mode,
            residuals,
            system_id,
            signal_id,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grs() {
        let input = "104148.00,1,2.6,2.2,-1.6,-1.1,-1.7,-1.5,5.8,1.7,,,,,1,1";
        let expected = Ok((
            "",
            GRSMessage {
                time: Some(NaiveTime::from_hms(10, 41, 48)),
                mode: Some(ComputationMethod::AfterGGA),
                residuals: [
                    Some(Meter(2.6)),
                    Some(Meter(2.2)),
                    Some(Meter(-1.6)),
                    Some(Meter(-1.1)),
                    Some(Meter(-1.7)),
                    Some(Meter(-1.5)),
                    Some(Meter(5.8)),
                    Some(Meter(1.7)),
                    None,
                    None,
                    None,
                    None,
                ],
                system_id: Some(1),
                signal_id: Some(1),
            },
        ));

        assert_eq!(expected, parse_grs(input));
    }
}
