#[derive(Debug, PartialEq)]
pub enum Fix {
    NoFix,
    AutonomousGNSSFix,
    DifferentialGNSSFix,
    RTKFixed,
    RTKFloat,
    EstimatedOrDeadReckoningFix,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    DataInvalid,
    DataValid,
}

#[derive(Debug, PartialEq)]
pub enum OperationMode {
    Manual,
    Automatic,
}

#[derive(Debug, PartialEq)]
pub enum ComputationMethod {
    InGGA,
    AfterGGA,
}

#[derive(Debug, PartialEq)]
pub enum NavigationMode {
    FixNo,
    Fix2D,
    Fix3D,
}

#[derive(Debug, PartialEq)]
pub enum NavigationalStatus {
    Safe,
    Caution,
    Unsafe,
    NotValid,
}

#[derive(Debug, PartialEq)]
pub enum SentenceType {
    Parametric,
    Encapsulation,
}

#[derive(Debug, PartialEq)]
pub enum MessageLevel {
    Error,
    Warning,
    Notice,
    User,
}
