#[derive(Debug, PartialEq)]
pub struct Knot(pub f64);

#[derive(Debug, PartialEq)]
pub enum WaterDistanceUnit {
    NauticalMile,
}

#[derive(Debug, PartialEq)]
pub enum CourseOverGroundUnit {
    DegreesTrue,
    DegreesMagnetic,
}

#[derive(Debug, PartialEq)]
pub enum SpeedOverGroundUnit {
    Knots,
    KilometersPerHour,
}
