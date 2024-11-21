use crate::game::{Coord, Direction};

#[derive(Copy, Clone)]
pub(crate) struct Ship {
    pub(crate) origin: Coord, // the ass of the ship
    pub(crate) length: i32,
    pub(crate) direction: Direction, // direction of which way the ship is facing from the POV of its ass
}

impl Ship {
    pub(crate) fn new(origin: Coord, length: i32, direction: Direction) -> Ship {
        Ship{
            origin,
            length,
            direction,
        }
    }

    pub(crate) fn is_collide(&self, coord: Coord) -> bool { // collision detection for a coord to see if the ship occupies the provided coord

        fn is_collide_check_right(coord: Coord, origin: Coord, length: i32) -> bool {
            let temp_coord = coord;
            let mut temp_origin = origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.x += 1;
            }

            false
        }

        fn is_collide_check_down(coord: Coord, origin: Coord, length: i32) -> bool {
            let temp_coord = coord;
            let mut temp_origin = origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.y += 1;
            }

            false
        }

        fn is_collide_check_left(coord: Coord, origin: Coord, length: i32) -> bool {
            let temp_coord = coord;
            let mut temp_origin = origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.x -= 1;
            }

            false
        }

        fn is_collide_check_up(coord: Coord, origin: Coord, length: i32) -> bool {
            let temp_coord = coord;
            let mut temp_origin = origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.y -= 1;
            }

            false
        }

        let mut is_collide_check : fn(Coord, Coord, i32) -> bool;
        match self.direction {
            Direction::Up => is_collide_check = is_collide_check_up,
            Direction::Down => is_collide_check = is_collide_check_down,
            Direction::Left => is_collide_check = is_collide_check_left,
            Direction::Right => is_collide_check = is_collide_check_right,
        }
        is_collide_check(coord, self.origin, self.length)

    }
}