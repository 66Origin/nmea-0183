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

#[derive(Debug, PartialEq)]
pub struct SatelliteInView {
    pub id: Option<u8>,
    pub elv: Option<u8>,
    pub az: Option<u16>,
    pub cno: Option<u8>,
}
