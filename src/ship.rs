use crate::game::{Coord, Direction};
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub(crate) struct Ship {
    pub(crate) origin: Coord, // the ass of the ship
    pub(crate) length: i32,
    pub(crate) direction: Direction, // direction of which way the ship is facing from the POV of its ass
}

impl Ship {
    pub(crate) fn new(origin: Coord, length: i32, direction: Direction) -> Ship {
        Ship{
            origin, // short for this.origin = origin
            length,
            direction,
        }
    }

    pub(crate) fn is_collide(self_ship: &Ship, coord: &Coord) -> bool { // collision detection for a coord to see if the ship occupies the provided coord

        // defining 4 functions for checks of the 4 direction the ship can face
        // 1 will be selected to be shoved into a variable below
        // look at the right function for comments as all 4 r the same thing with minor difference
        // main function logic happens below

        // checker for when the ship is facing right
        // will be shoved into a var below
        fn is_collide_check_right(coord: &Coord, origin: &Coord, length: i32) -> bool {
            let temp_coord = *coord;
            let mut temp_origin = *origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true; // returns the second it sees the tile provided is the same tile the ship takes up
                }
                temp_origin.x += 1;
            }

            false
        }

        // checker for when the ship is facing down
        // will be shoved into a var below
        fn is_collide_check_down(coord: &Coord, origin: &Coord, length: i32) -> bool {
            let temp_coord = *coord;
            let mut temp_origin = *origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.y += 1;
            }

            false
        }

        // checker for when the ship is facing left
        // will be shoved into a var below
        fn is_collide_check_left(coord: &Coord, origin: &Coord, length: i32) -> bool {
            let temp_coord = *coord;
            let mut temp_origin = *origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.x -= 1;
            }

            false
        }

        // checker for when the ship is facing up
        // will be shoved into a var below
        fn is_collide_check_up(coord: &Coord, origin: &Coord, length: i32) -> bool {
            let temp_coord = *coord;
            let mut temp_origin = *origin;
            for _i in 0..length {
                if temp_coord == temp_origin {
                    return true;
                }
                temp_origin.y -= 1;
            }

            false
        }

        // main function logic starts here

        // this is a var that holds a function that takes in a Coord struct passed by reference, another Coord struct passed by reference, and an i32 that returns a bool
        let is_collide_check : fn(&Coord, &Coord, i32) -> bool;
        match self_ship.direction { // mapping the direction to fill in the var with the correct function to run
            Direction::Up => is_collide_check = is_collide_check_up,
            Direction::Down => is_collide_check = is_collide_check_down,
            Direction::Left => is_collide_check = is_collide_check_left,
            Direction::Right => is_collide_check = is_collide_check_right,
        }

        // run the function
        is_collide_check(coord, &self_ship.origin, self_ship.length)

    }

    // function that returns a list of tiles that has a ship in it
    // in itself is a pure function
    pub(crate) fn coord_list(&self) -> Vec<Coord> {
        let mut output  = Vec::new();
        let mut temp_coord = self.origin;

        // 4 functions dealing with the ships direction again
        // u know the drill at this point
        fn move_left(coord: &mut Coord) {
            coord.x -= 1;
        }
        fn move_right(coord: &mut Coord) {
            coord.x += 1;
        }
        fn move_up(coord: &mut Coord) {
            coord.y -= 1;
        }
        fn move_down(coord: &mut Coord) {
            coord.y += 1;
        }

        // main function logic starts here

        let mut move_func : fn(&mut Coord);

        for _i in 0..self.length {

            output.push(temp_coord);

            match self.direction {
                Direction::Up => move_func = move_up,
                Direction::Right => move_func = move_right,
                Direction::Left => move_func = move_left,
                Direction::Down => move_func = move_down,
            }

            move_func(&mut temp_coord);

        }
        output
    }
}

impl fmt::Display for Ship { // essentially a toString
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x:{} y:{} dir:{} len:{}", self.origin.x, self.origin.y, self.direction, self.length)
    }
}