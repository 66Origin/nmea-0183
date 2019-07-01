#[derive(Debug, PartialEq)]
/// Latitudes are provided unsigned.
/// NorthSouth defines the direction for the latitude.
pub enum NorthSouth {
    North,
    South,
}

#[derive(Debug, PartialEq)]
/// Longitudes are provided unsigned.
/// EastWest defines the direction for the longitude.
pub enum EastWest {
    East,
    West,
}
