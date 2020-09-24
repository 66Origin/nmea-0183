use arrayvec::ArrayVec;

use crate::fields::distance::Degree;
use crate::fields::parameter::DBHZ;

#[derive(Debug, PartialEq)]
/// The type of the transmitting unit that sent the message
pub enum Talker {
    /// Independent AIS Base Station
    IndependentAISBaseStation,
    /// Dependent AIS Base Station
    DependentAISBaseStation,
    /// Autopilot - General
    AutopilotGeneral,
    /// Autopilot - Magnetic
    AutopilotMagnetic,
    /// BeiDou (China)
    BeiDou,
    /// Bridge navigational watch alarm system
    BridgeNavigationalWatchAlarmSystem,
    /// Computer - Programmed Calculator (obsolete)
    ComputerProgrammedCalculator,
    /// Communications - Digital Selective Calling (DSC)
    DigitalSelectiveCalling,
    /// Computer - Memory Data (obsolete)
    ComputerMemoryData,
    /// Communications - Satellite
    CommunicationsSatellite,
    /// Communications - Radio-Telephone (MF/HF)
    CommunicationsRadioTelephoneMFHF,
    /// Communications - Radio-Telephone (VHF)
    CommunicationsRadioTelephoneVHF,
    /// Communications - Scanning Receiver
    CommunicationsScanningReceiver,
    /// DECCA Navigation (obsolete),
    DECCA,
    /// Direction Finder
    DirectionFinder,
    /// Velocity Sensor, Speed Log, Water, Magnetic
    VelocitySensorSpeedLogWaterMagnetic,
    /// Duplex repeater station
    DuplexRepeaterStation,
    /// Electronic Chart Display & Information System (ECDIS)
    ElectronicChartDisplayAndInformationSystem,
    /// Emergency Position Indicating Beacon (EPIRB)
    EmergencyPositionIndicatingBeacon,
    /// Engine Room Monitoring Systems
    EngineRoomMonitoringSystems,
    /// Galileo Positioning System
    Galileo,
    /// GLONASS, according to IEIC 61162-1
    GLONASS,
    /// Mixed GPS and GLONASS data, according to IEIC 61162-1
    GPSGLONASS,
    /// Global Positioning System
    GPS,
    /// Heading - Magnetic Compass
    HeadingMagneticCompass,
    /// Heading - North Seeking Gyro
    HeadingNorthSeekingGyro,
    /// Heading - Non North Seeking Gyro
    HeadingNonNorthSeekingGyro,
    /// Integrated Instrumentation
    IntegratedInstrumentation,
    /// Integrated Navigation
    IntegratedNavigation,
    /// Loran A receiver (obsolete)
    LoranAReceiver,
    /// Loran-C receiver (obsolete)
    LoranCReceiver,
    /// Microwave Positioning System (obsolete)
    MicrowavePositioningSystem,
    /// Navigation light controller,
    NavigationLightController,
    /// OMEGA Navigation System (obsolete)
    OMEGANavigationSystem,
    /// Distress Alarm System (obsolete)
    DistressAlarmSystem,
    /// RADAR and/or ARPA
    RADAROrARPA,
    /// Sounder, Depth
    SounderDepth,
    /// Electronic Positioning System, other/general
    ElectronicPositioningSystem,
    /// Sounder, Scanning
    SounderScanning,
    /// Turn Rate Indicator
    TurnRateIndicator,
    /// TRANSIT Navigation System
    TRANSITNavigationSystem,
    /// User Configured
    U0,
    /// User Configured
    U1,
    /// User Configured
    U2,
    /// User Configured
    U3,
    /// User Configured
    U4,
    /// User Configured
    U5,
    /// User Configured
    U6,
    /// User Configured
    U7,
    /// User Configured
    U8,
    /// User Configured
    U9,
    /// Microprocessor controller
    MicroprocessorController,
    /// QZSS regional GPS augmentation system (Japan)
    QZSS,
    /// Velocity Sensor, Doppler, other/general
    VelocitySensorDoppler,
    /// Velocity Sensor, Speed Log, Water, Mechanical,
    VelocitySensorSpeedLogWaterMechanical,
    /// Weather Instruments
    WeatherInstruments,
    /// Transducer - Temperature (obsolete)
    TransducerTemperature,
    /// Transducer - Displacement, Angular or Linear (obsolete)
    TransducterDisplacementAngularOrLinear,
    /// Transducer - Frequency (obsolete)
    TransducerFrequency,
    /// Transducer - Level (obsolete)
    TransducerLevel,
    /// Transducer - Pressure (obsolete)
    TransducerPressure,
    /// Transducer - Flow Rate (obsolete)
    TransducerFlowRate,
    /// Transducer - Tachometer (obsolete)
    TransducerTachometer,
    /// Transducer - Volume (obsolete)
    TransducerVolume,
    /// Transducer
    Transducer,
    /// Timekeeper - Atomic Clock
    TimekeeperAtomicClock,
    /// Timekeeper - Chronometer
    TimekeeperChronometer,
    /// Timekeeper - Quartz
    TimekeeperQuartz,
    /// Timekeeper - Radio Update, WWV or WWVH
    TimekeeperRadioUpdate,
}

#[derive(Debug, PartialEq)]
/// Represents a Satellite
/// With its position
pub struct SatelliteInView {
    pub id: Option<u8>,
    /// Elevation
    pub elv: Option<Degree>,
    /// Azimuth
    pub az: Option<Degree>,
    /// Signal strength
    pub cno: Option<DBHZ>,
}

/// List of satellites in view. Maximum of 4 as per GSV definition.
pub type SatelliteInViewList = ArrayVec<[SatelliteInView; 4]>;
