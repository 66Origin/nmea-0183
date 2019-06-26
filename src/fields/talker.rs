use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Talker {
    IndependentAISBaseStation,                  // Independent AIS Base Station
    DependentAISBaseStation,                    // Dependent AIS Base Station
    AutopilotGeneral,                           // Autopilot - General
    AutopilotMagnetic,                          // Autopilot - Magnetic
    BeiDou,                                     // BeiDou (China)
    BridgeNavigationalWatchAlarmSystem,         // Bridge navigational watch alarm system
    ComputerProgrammedCalculator,               // Computer - Programmed Calculator (obsolete)
    DigitalSelectiveCalling,                    // Communications - Digital Selective Calling (DSC)
    ComputerMemoryData,                         // Computer - Memory Data (obsolete)
    CommunicationsSatellite,                    // Communications - Satellite
    CommunicationsRadioTelephoneMFHF,           // Communications - Radio-Telephone (MF/HF)
    CommunicationsRadioTelephoneVHF,            // Communications - Radio-Telephone (VHF)
    CommunicationsScanningReceiver,             // Communications - Scanning Receiver
    DECCA,                                      // DECCA Navigation (obsolete),
    DirectionFinder,                            // Direction Finder
    VelocitySensorSpeedLogWaterMagnetic,        // Velocity Sensor, Speed Log, Water, Magnetic
    DuplexRepeaterStation,                      // Duplex repeater station
    ElectronicChartDisplayAndInformationSystem, // Electronic Chart Display & Information System (ECDIS)
    EmergencyPositionIndicatingBeacon,          // Emergency Position Indicating Beacon (EPIRB)
    EngineRoomMonitoringSystems,                // Engine Room Monitoring Systems
    Galileo,                                    // Galileo Positioning System
    GLONASS,                                    // GLONASS, according to IEIC 61162-1
    GPSGLONASS,                  // Mixed GPS and GLONASS data, according to IEIC 61162-1
    GPS,                         // Global Positioning System
    HeadingMagneticCompass,      // Heading - Magnetic Compass
    HeadingNorthSeekingGyro,     // Heading - North Seeking Gyro
    HeadingNonNorthSeekingGyro,  // Heading - Non North Seeking Gyro
    IntegratedInstrumentation,   // Integrated Instrumentation
    IntegratedNavigation,        // Integrated Navigation
    LoranAReceiver,              // Loran A receiver (obsolete)
    LoranCReceiver,              // Loran-C receiver (obsolete)
    MicrowavePositioningSystem,  // Microwave Positioning System (obsolete)
    NavigationLightController,   // Navigation light controller,
    OMEGANavigationSystem,       // OMEGA Navigation System (obsolete)
    DistressAlarmSystem,         // Distress Alarm System (obsolete)
    RADAROrARPA,                 // RADAR and/or ARPA
    SounderDepth,                // Sounder, Depth
    ElectronicPositioningSystem, // Electronic Positioning System, other/general
    SounderScanning,             // Sounder, Scanning
    TurnRateIndicator,           // Turn Rate Indicator
    TRANSITNavigationSystem,     // TRANSIT Navigation System
    U0,                          // User Configured
    U1,                          // User Configured
    U2,                          // User Configured
    U3,                          // User Configured
    U4,                          // User Configured
    U5,                          // User Configured
    U6,                          // User Configured
    U7,                          // User Configured
    U8,                          // User Configured
    U9,                          // User Configured
    MicroprocessorController,    // Microprocessor controller
    QZSS,                        // QZSS regional GPS augmentation system (Japan)
    VelocitySensorDoppler,       // Velocity Sensor, Doppler, other/general
    VelocitySensorSpeedLogWaterMechanical, // Velocity Sensor, Speed Log, Water, Mechanical,
    WeatherInstruments,          // Weather Instruments
    TransducerTemperature,       // Transducer - Temperature (obsolete)
    TransducterDisplacementAngularOrLinear, // Transducer - Displacement, Angular or Linear (obsolete)
    TransducerFrequency,                    // Transducer - Frequency (obsolete)
    TransducerLevel,                        // Transducer - Level (obsolete)
    TransducerPressure,                     // Transducer - Pressure (obsolete)
    TransducerFlowRate,                     // Transducer - Flow Rate (obsolete)
    TransducerTachometer,                   // Transducer - Tachometer (obsolete)
    TransducerVolume,                       // Transducer - Volume (obsolete)
    Transducer,                             // Transducer
    TimekeeperAtomicClock,                  // Timekeeper - Atomic Clock
    TimekeeperChronometer,                  // Timekeeper - Chronometer
    TimekeeperQuartz,                       // Timekeeper - Quartz
    TimekeeperRadioUpdate,                  // Timekeeper - Radio Update, WWV or WWVH
}

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

#[cfg(test)]
mod talker_tests {
    use super::*;

    #[test]
    fn test_parse_talker() {
        let input = "HNtest";
        let expected_output = Ok(("test", Talker::HeadingNonNorthSeekingGyro));
        assert_eq!(expected_output, parse_talker(input));
    }

    #[test]
    fn test_parse_wrong_parse_talker() {
        let input = "thisisnotatalker";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        assert_eq!(expected_output, parse_talker(input));
    }

    #[test]
    fn test_parse_not_enough_characters() {
        let input = "*foo bar";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        assert_eq!(expected_output, parse_talker(input));
    }

    #[test]
    fn test_parse_talker_empty_input() {
        let input = "";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        assert_eq!(expected_output, parse_talker(input));
    }
}
