use std::cmp::PartialEq;
use std::fmt;

#[derive(Copy, Clone)] // enables the enum to be copied and cloned
pub(crate) enum Space {
    Unknown,
    Hit,
    Miss,
    Forfeit,
}

//tostring method for enums using fmt::display as implementation
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Space::Unknown => write!(f, " "),
            Space::Hit => write!(f, "X"),
            Space::Miss => write!(f, "O"),
            Space::Forfeit => write!(f, "i"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Coord { // coord data struct to represent coord in grid
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coord {
    pub(crate) fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl PartialEq for Coord { // equivalent to .equals() in C#
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// impl fmt::Display for Direction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Direction::Left => write!(f, "0"),
//             Direction::Right => write!(f, "2"),
//             Direction::Up => write!(f, "2"),
//             Direction::Down => write!(f, "3"),
//         }
//     }
// }

struct Game {
    hits: i32,
    misses: i32,
    shots_taken: i32,

}

impl Game {
    fn new() -> Game {
        Game{
            hits : 0,
            misses : 0,
            shots_taken: 0,

        }
    }

    fn main(){

    }
}

