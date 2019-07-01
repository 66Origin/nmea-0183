#[derive(Debug, PartialEq)]
/// Nautical mile per hour
pub struct Knot(pub f64);

#[derive(Debug, PartialEq)]
/// The unit used to display the distance over water
pub enum WaterDistanceUnit {
    NauticalMile,
}

#[derive(Debug, PartialEq)]
/// The unit used to display the Course over Ground
pub enum CourseOverGroundUnit {
    /// Degrees using the true North
    DegreesTrue,
    /// Degrees using the magnetic North
    DegreesMagnetic,
}

#[derive(Debug, PartialEq)]
/// The unit used to display the speed over Ground
pub enum SpeedOverGroundUnit {
    Knots,
    KilometersPerHour,
}
