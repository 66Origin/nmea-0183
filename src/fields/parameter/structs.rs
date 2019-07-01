#[derive(Debug, PartialEq)]
/// GPS quality indicator
pub enum Fix {
    /// Fix not valid
    NoFix,
    /// GPS Fix
    AutonomousGNSSFix,
    /// Differential GPS fix, OmniSTAR VBS
    DifferentialGNSSFix,
    /// Real-Time Kinematic, fixed integers
    RTKFixed,
    /// Real-Time Kinematic, float integers, OmniSTAR XP/HP or Location RTK
    RTKFloat,
    /// U-BLOX dead reckoning fix
    EstimatedOrDeadReckoningFix,
}

#[derive(Debug, PartialEq)]
/// Defines if the provided data is valid or not.
pub enum Status {
    DataInvalid,
    DataValid,
}

#[derive(Debug, PartialEq)]
/// Defines if the device can switch 2D or 3D mode
pub enum OperationMode {
    /// Manual—forced to operate in 2D or 3D mode
    Manual,
    /// 2D Automatic—allowed to automatically switch 2D/3D
    Automatic,
}

#[derive(Debug, PartialEq)]
/// A computation method is provided in GRS messages
/// to declare how Range Residuals have been computed.
pub enum ComputationMethod {
    /// The Range Residual has been used to compute the GGA position
    InGGA,
    /// The Range Residual has been recomputed after
    /// the GGA position was computed.
    AfterGGA,
}

#[derive(Debug, PartialEq)]
/// Defines if a Fix has been used
pub enum NavigationMode {
    /// No fix available
    FixNo,
    /// Less than 4 satellites have been found,
    /// a constant altitude has been used to compensate the missing 4th satellite.
    Fix2D,
    /// 4 or more satellites have been found, optimal fix.
    Fix3D,
}

#[derive(Debug, PartialEq)]
/// Specifies how accurate the Navigation data is,
/// and if it is considered safe to use.
pub enum NavigationalStatus {
    Safe,
    Caution,
    Unsafe,
    NotValid,
}

#[derive(Debug, PartialEq)]
/// The sentence type is defined
/// in the first character of an NMEA sentence.
pub enum SentenceType {
    /// Conventional field delimited messages
    Parametric,
    /// Messages that have special encapsulation (=fields) in them
    Encapsulation,
}

#[derive(Debug, PartialEq)]
/// TXT transmission emergency level
pub enum MessageLevel {
    Error,
    Warning,
    Notice,
    User,
}

#[derive(Debug, PartialEq)]
/// Carrier-to-noise-density ratio
pub struct DBHZ(pub f64);
