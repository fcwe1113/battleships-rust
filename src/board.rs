use rand::Rng;
use crate::game::Coord;
use crate::ship::Ship;

pub struct Board{ //data for Board implementation
    pub(crate) grid : [[crate::game::Space;10];10], // a 10 by 10 array holding Space structs
    pub(crate) ships: Vec<Ship>, // a Vector(essentially java ArrayList) holding the Ship struct
}

impl Board{

    pub(crate) fn new() -> Board{ // constructor

        Board {
            grid: [[crate::game::Space::Unknown; 10]; 10], // create a 10 by 10 array initialized with unknown
            ships: Board::ship_gen(), // run the ship_gen function to fill the Vector with ships
        }

    }

    // the ship gen function, should only be called by Board::new()
    fn ship_gen() -> Vec<Ship> {

        let mut ship_list_output: Vec<Ship> = Vec::new();
        let lengths = [2, 3, 3, 4, 5]; // hardcoded ship lengths

        // this is now a pure function
        fn new_ship(length: i32, list: &Vec<Ship>) -> Ship {

            // defining 4 check functions for 4 directions
            // just look at the left function for comments they are essentially the same with minimal changes
            // main function logic starts below

            // the check function if the newly generated ship is facing left
            // would be shoved into a var below
            fn new_ship_left_check(coord: Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.x < 0 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if Ship::is_collide(&list[i], &temp_coord) { // check each tile of the ship will it take up to see if anythings occupying it
                            return false;
                        }
                    }
                    temp_coord.x -= 1;
                }

                true
            }

            // the check function if the newly generated ship is facing right
            // would be shoved into a var below
            fn new_ship_right_check(coord: Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.x > 9 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if Ship::is_collide(&list[i], &temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.x += 1;
                }

                true
            }

            // the check function if the newly generated ship is facing up
            // would be shoved into a var below
            fn new_ship_up_check(coord: Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.y < 0 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if Ship::is_collide(&list[i], &temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.y -= 1;
                }

                true
            }


            // the check function if the newly generated ship is facing down
            // would be shoved into a var below
            fn new_ship_down_check(coord: Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.y > 9 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if Ship::is_collide(&list[i], &temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.y += 1;
                }

                true
            }

            // main function logic starts here
            let mut done: bool = false;
            let mut output: Ship = Ship { // make a dummy ship to shut the compiler up
                origin: Coord { x: -1, y: -1 },
                length,
                direction: crate::game::Direction::Left,
            };
            while !done {

                //do the rng here
                //both starting coord and direction
                //the 4 funcs 2 for checking if pos valid or not
                let mut rng = rand::thread_rng();
                // randomizing the origin coord of the ship
                let coord: Coord = Coord::new(rng.gen_range(0..10), rng.gen_range(0..10));
                let direction: crate::game::Direction = match rng.gen_range(0..4) {
                    // randomizing the direction of the ship and turn the rng number into a direction enum
                    0 => crate::game::Direction::Right,
                    1 => crate::game::Direction::Down,
                    2 => crate::game::Direction::Left,
                    3 => crate::game::Direction::Up,
                    _ => crate::game::Direction::Right //almost never happens but who fking knows
                };

                // this is a variable that stores a function that takes in a Coord struct , a i32 and a Vector holding Ship structs by reference returning a bool
                let new_ship_check: fn(Coord, i32, &Vec<Ship>) -> bool;
                match direction {
                    crate::game::Direction::Up => { new_ship_check = new_ship_up_check; },
                    crate::game::Direction::Down => { new_ship_check = new_ship_down_check; },
                    crate::game::Direction::Left => { new_ship_check = new_ship_left_check; },
                    crate::game::Direction::Right => { new_ship_check = new_ship_right_check; },
                }

                // make the new Ship with the randomized information
                output = Ship::new(coord, length, direction);
                // check the new Ship to see if the placement is valid
                done = new_ship_check(coord, length, list);
            }
            output // returns the ship
        }

        for length in lengths{

            // append ships into the newly generated ship list
            // remember the validity check is in the new_ship() already
            ship_list_output.push(new_ship(length, &ship_list_output));
        }
        ship_list_output
    }

    // print board function
    pub(crate) fn print_board(&self){ //pass by reference because we dont need ownership of it

        print!("       |");
        for i in 0..10 {
            if i != 9 {
                print!(" ");
            }
            print!("{} |", i + 1)
        }
        println!();

        for i in 0..10 {
            println!("    --------------------------------------------");
            if i != 9 {
                print!(" ");
            }
            println!("    {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |", i + 1, self.grid[i][0],self.grid[i][1],self.grid[i][2],self.grid[i][3],self.grid[i][4],self.grid[i][5],self.grid[i][6],self.grid[i][7],self.grid[i][8],self.grid[i][9])
            //without the above tostring all the grid would be printing as their enums
        }
        println!("    --------------------------------------------");
    }

    // the check if the provided coord has a ship hiding in there function
    // pure function here
    // it takes variables, gives an output according to the input, no randomizing or user input inside
    pub(crate) fn check_hit(&self, coord : &Coord) -> bool{
        let mut output = false;
        for ship in &self.ships{
            if Ship::is_collide(ship, coord) { // its just proccing the ship struct's collision check
                output = true;
            }
        }
        output
    }

    // total ship length function for the win con
    pub(crate) fn total_ship_length(&self) -> i32{
        let mut total = 0;
        for ship in &self.ships {
            total += ship.length;
        }
        total
    }
}