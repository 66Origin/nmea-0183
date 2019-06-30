use super::structs::*;
use crate::parser_utils::*;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_talker(input: &str) -> IResult<&str, Talker> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (maybe_talker, remaining) = input.split_at(2);
    match maybe_talker {
        "AB" => Ok((remaining, Talker::IndependentAISBaseStation)),
        "AD" => Ok((remaining, Talker::DependentAISBaseStation)),
        "AG" => Ok((remaining, Talker::AutopilotGeneral)),
        "AP" => Ok((remaining, Talker::AutopilotMagnetic)),
        "BD" => Ok((remaining, Talker::BeiDou)),
        "BN" => Ok((remaining, Talker::BridgeNavigationalWatchAlarmSystem)),
        "CC" => Ok((remaining, Talker::ComputerProgrammedCalculator)),
        "CD" => Ok((remaining, Talker::DigitalSelectiveCalling)),
        "CM" => Ok((remaining, Talker::ComputerMemoryData)),
        "CS" => Ok((remaining, Talker::CommunicationsSatellite)),
        "CT" => Ok((remaining, Talker::CommunicationsRadioTelephoneMFHF)),
        "CV" => Ok((remaining, Talker::CommunicationsRadioTelephoneVHF)),
        "CX" => Ok((remaining, Talker::CommunicationsScanningReceiver)),
        "DE" => Ok((remaining, Talker::DECCA)),
        "DF" => Ok((remaining, Talker::DirectionFinder)),
        "DM" => Ok((remaining, Talker::VelocitySensorSpeedLogWaterMagnetic)),
        "DU" => Ok((remaining, Talker::DuplexRepeaterStation)),
        "EC" => Ok((
            remaining,
            Talker::ElectronicChartDisplayAndInformationSystem,
        )),
        "EP" => Ok((remaining, Talker::EmergencyPositionIndicatingBeacon)),
        "ER" => Ok((remaining, Talker::EngineRoomMonitoringSystems)),
        "GA" => Ok((remaining, Talker::Galileo)),
        "GB" => Ok((remaining, Talker::BeiDou)),
        "GL" => Ok((remaining, Talker::GLONASS)),
        "GN" => Ok((remaining, Talker::GPSGLONASS)),
        "GP" => Ok((remaining, Talker::GPS)),
        "HC" => Ok((remaining, Talker::HeadingMagneticCompass)),
        "HE" => Ok((remaining, Talker::HeadingNonNorthSeekingGyro)),
        "HN" => Ok((remaining, Talker::HeadingNonNorthSeekingGyro)),
        "II" => Ok((remaining, Talker::IntegratedInstrumentation)),
        "IN" => Ok((remaining, Talker::IntegratedNavigation)),
        "LA" => Ok((remaining, Talker::LoranAReceiver)),
        "LC" => Ok((remaining, Talker::LoranCReceiver)),
        "MP" => Ok((remaining, Talker::MicrowavePositioningSystem)),
        "NL" => Ok((remaining, Talker::NavigationLightController)),
        "OM" => Ok((remaining, Talker::OMEGANavigationSystem)),
        "OS" => Ok((remaining, Talker::DistressAlarmSystem)),
        "QZ" => Ok((remaining, Talker::QZSS)),
        "RA" => Ok((remaining, Talker::RADAROrARPA)),
        "SD" => Ok((remaining, Talker::SounderDepth)),
        "SN" => Ok((remaining, Talker::ElectronicPositioningSystem)),
        "SS" => Ok((remaining, Talker::SounderScanning)),
        "TI" => Ok((remaining, Talker::TurnRateIndicator)),
        "TR" => Ok((remaining, Talker::TRANSITNavigationSystem)),
        "U0" => Ok((remaining, Talker::U0)),
        "U1" => Ok((remaining, Talker::U1)),
        "U2" => Ok((remaining, Talker::U2)),
        "U3" => Ok((remaining, Talker::U3)),
        "U4" => Ok((remaining, Talker::U4)),
        "U5" => Ok((remaining, Talker::U5)),
        "U6" => Ok((remaining, Talker::U6)),
        "U7" => Ok((remaining, Talker::U7)),
        "U8" => Ok((remaining, Talker::U8)),
        "U9" => Ok((remaining, Talker::U9)),
        "UP" => Ok((remaining, Talker::MicroprocessorController)),
        "VD" => Ok((remaining, Talker::VelocitySensorDoppler)),
        "VW" => Ok((remaining, Talker::VelocitySensorSpeedLogWaterMechanical)),
        "WI" => Ok((remaining, Talker::WeatherInstruments)),
        "YC" => Ok((remaining, Talker::TransducerTemperature)),
        "YD" => Ok((remaining, Talker::TransducterDisplacementAngularOrLinear)),
        "YF" => Ok((remaining, Talker::TransducerFrequency)),
        "YL" => Ok((remaining, Talker::TransducerLevel)),
        "YP" => Ok((remaining, Talker::TransducerPressure)),
        "YR" => Ok((remaining, Talker::TransducerFlowRate)),
        "YT" => Ok((remaining, Talker::TransducerTachometer)),
        "YV" => Ok((remaining, Talker::TransducerVolume)),
        "YX" => Ok((remaining, Talker::Transducer)),
        "ZA" => Ok((remaining, Talker::TimekeeperAtomicClock)),
        "ZC" => Ok((remaining, Talker::TimekeeperChronometer)),
        "ZQ" => Ok((remaining, Talker::TimekeeperQuartz)),
        "ZV" => Ok((remaining, Talker::TimekeeperRadioUpdate)),
        _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
    }
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

pub fn parse_num_satellites(input: &str) -> IResult<&str, Option<u8>> {
    parse_u8(input)
}

pub fn parse_station(input: &str) -> IResult<&str, Option<u8>> {
    parse_u8(input).or_else(|err| match err {
        nom::Err::Failure((input, nom::error::ErrorKind::Complete)) => Ok((input, None)),
        _ => Err(err),
    })
}

pub fn parse_system(input: &str) -> IResult<&str, Option<u8>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    parse_u8(input)
}

pub fn parse_signal(input: &str) -> IResult<&str, Option<u8>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    parse_u8(input)
}
