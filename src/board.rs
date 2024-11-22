use rand::Rng;
use crate::ship::Ship;

pub struct Board{ //data for Board implementation
    pub(crate) grid : [[crate::game::Space;10];10],
    pub(crate) ships: Vec<Ship>,
}

impl Board{

    pub(crate) fn new() -> Board{

        Board {
            grid: [[crate::game::Space::Unknown; 10]; 10],
            ships: Board::ship_gen(),
        }

    }

    fn ship_gen() -> Vec<Ship> {

        let mut ship_list_output: Vec<Ship> = Vec::new();
        let lengths = [2, 3, 3, 4, 5];

        fn new_ship(length: i32, list: &Vec<Ship>) -> Ship {
            fn new_ship_left_check(coord: crate::game::Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.x < 0 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if list[i].is_collide(temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.x -= 1;
                }

                true
            }

            fn new_ship_right_check(coord: crate::game::Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.x > 9 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if list[i].is_collide(temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.x += 1;
                }

                true
            }

            fn new_ship_up_check(coord: crate::game::Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.y < 0 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if list[i].is_collide(temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.y -= 1;
                }

                true
            }

            fn new_ship_down_check(coord: crate::game::Coord, size: i32, list: &Vec<Ship>) -> bool {
                let mut temp_coord = coord;

                for _i in 0..size {
                    if temp_coord.y > 9 {
                        return false;
                    }

                    for i in 0..list.len() {
                        if list[i].is_collide(temp_coord) {
                            return false;
                        }
                    }
                    temp_coord.y += 1;
                }

                true
            }

            let mut done: bool = false;
            let mut output: Ship = Ship {
                origin: crate::game::Coord { x: -1, y: -1 },
                length,
                direction: crate::game::Direction::Left,
            };
            while !done {

                //do the rng here
                //both starting coord and direction
                //the 4 funcs 2 for checking if pos valid or not
                let mut rng = rand::thread_rng();
                let coord: crate::game::Coord = crate::game::Coord::new(rng.gen_range(0..10), rng.gen_range(0..10));
                let direction: crate::game::Direction = match rng.gen_range(0..4) {
                    0 => crate::game::Direction::Right,
                    1 => crate::game::Direction::Down,
                    2 => crate::game::Direction::Left,
                    3 => crate::game::Direction::Up,
                    _ => crate::game::Direction::Right //almost never happens but who fking knows
                };
                let mut new_ship_check: fn(crate::game::Coord, i32, &Vec<Ship>) -> bool;
                match direction {
                    crate::game::Direction::Up => { new_ship_check = new_ship_up_check; },
                    crate::game::Direction::Down => { new_ship_check = new_ship_down_check; },
                    crate::game::Direction::Left => { new_ship_check = new_ship_left_check; },
                    crate::game::Direction::Right => { new_ship_check = new_ship_right_check; },
                }

                output = Ship::new(coord, length, direction);
                done = new_ship_check(coord, length, list);
            }
            output
        }

        for length in lengths{
            // println!("{:?}", lengths);

            ship_list_output.push(new_ship(length, &ship_list_output));
            // for ship in &ship_list_output {
            //     println!("{} {} {} {} {}",ship.origin.x, ship.origin.y, ship.length, ship.direction, length);
            // }
        }
        ship_list_output
    }

    pub(crate) fn print_board(&self){ //pass by reference because we dont need ownership of it
        // let grid = self.grid.clone();


        for i in 0..10 {
            println!("       -----------------------------------------");
            println!("       | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",self.grid[i][0],self.grid[i][1],self.grid[i][2],self.grid[i][3],self.grid[i][4],self.grid[i][5],self.grid[i][6],self.grid[i][7],self.grid[i][8],self.grid[i][9])
            //without the above tostring all the grid would be printing as their enums
        }
        println!("       -----------------------------------------");
    }
}