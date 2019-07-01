#[derive(Debug, PartialEq)]
/// Angle
pub struct Degree(pub f64);

#[derive(Debug, PartialEq)]
/// 1/60th of a Degree
pub struct Minute(pub f64);

#[derive(Debug, PartialEq)]
/// 1/60th of a Minute
pub struct Second(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
/// The base unit of length in the International System of Units (SI)
pub struct Meter(pub f64);
