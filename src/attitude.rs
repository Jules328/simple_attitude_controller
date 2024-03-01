use std::ops;
use std::fmt::{Display, Formatter, Result, Debug};

/// Tuple Struct to represent x, y, z coordinates of satellite attitude 
#[derive(Debug, PartialEq)]
pub struct Attitude(pub i32, pub i32, pub i32);

impl Attitude {
    /// Creates a new attitude of all zeroes
    pub fn new() -> Self {
        Attitude(0, 0, 0)
    }

    /// returns string of the planet's name that the satellite is pointing towards
    pub fn get_planet(&self) -> String { 
        match *self {
            Attitude(x, y, z) if x > 0 && y > 0 && z > 0 => "GRACE", // +x +y +z
            Attitude(x, y, z) if x > 0 && y > 0 && z < 0 => "PRICE", // +x +y -z
            Attitude(x, y, z) if x > 0 && y < 0 && z > 0 => "BRAY",  // +x -y +z
            Attitude(x, y, z) if x > 0 && y < 0 && z < 0 => "MIG",   // +x -y -z
            Attitude(x, y, z) if x < 0 && y > 0 && z > 0 => "WIEM",  // -x +y +z
            Attitude(x, y, z) if x < 0 && y > 0 && z < 0 => "MROW",  // -x +y -z
            Attitude(x, y, z) if x < 0 && y < 0 && z > 0 => "TURK",  // -x -y +z
            Attitude(x, y, z) if x < 0 && y < 0 && z < 0 => "SEBAS", // -x -y -z
            _ => "UNKNOWN" // axes and origin
        }.to_string()
    }
}

impl ops::Add<Attitude> for Attitude {
    type Output = Attitude;

    fn add(self, rhs: Self) -> Self{
        Attitude(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Attitude> for Attitude {
    fn add_assign(&mut self, rhs: Attitude) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

// lets the attitude be displayed with println!
impl Display for Attitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attitude_display() {
        let att = Attitude(1, 2, 3);
        let s = format!("{att}");
        assert_eq!(s, "1, 2, 3");
    }

    #[test]
    fn add_attitude() {
        let att1 = Attitude(1, 2, 3);
        let att2 = Attitude(4, 5, 6);
        assert_eq!(att1 + att2, Attitude(5, 7, 9));
    }

    #[test]
    fn add_assign_attitude() {
        let mut att = Attitude(1, 2, 3);
        att += Attitude(4, 5, 6);
        assert_eq!(att, Attitude(5, 7, 9));
    }

    #[test]
    fn get_planet() {
        assert_eq!(Attitude( 1, 1, 1).get_planet(), "GRACE");   // +x +y +z
        assert_eq!(Attitude( 1, 1,-1).get_planet(), "PRICE");   // +x +y -z
        assert_eq!(Attitude( 1,-1, 1).get_planet(), "BRAY");    // +x -y +z
        assert_eq!(Attitude( 1,-1,-1).get_planet(), "MIG");     // +x -y -z
        assert_eq!(Attitude(-1, 1, 1).get_planet(), "WIEM");    // -x +y +z
        assert_eq!(Attitude(-1, 1,-1).get_planet(), "MROW");    // -x +y -z
        assert_eq!(Attitude(-1,-1, 1).get_planet(), "TURK");    // -x -y +z
        assert_eq!(Attitude(-1,-1,-1).get_planet(), "SEBAS");   // -x -y -z

        assert_eq!(Attitude( 0, 0, 0).get_planet(), "UNKNOWN"); // origin
        assert_eq!(Attitude( 1, 0, 0).get_planet(), "UNKNOWN"); // +x axis
        assert_eq!(Attitude(-1, 0, 0).get_planet(), "UNKNOWN"); // -x axis
        assert_eq!(Attitude( 0, 1, 0).get_planet(), "UNKNOWN"); // +y axis
        assert_eq!(Attitude( 0,-1, 0).get_planet(), "UNKNOWN"); // -y axis
        assert_eq!(Attitude( 0, 0, 1).get_planet(), "UNKNOWN"); // +z axis
        assert_eq!(Attitude( 0, 0,-1).get_planet(), "UNKNOWN"); // -z axis
    }
}