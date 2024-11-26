use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Display;
use std::io::stdin;
use crate::board::Board;

#[derive(Copy, Clone)] // enables the enum to be copied and cloned
pub(crate) enum Space {
    Unknown,
    Hit,
    Miss,
    Forfeit,
    Targeting,
}

//tostring method for enums using fmt::display as implementation
impl Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Space::Unknown => write!(f, " "),
            Space::Hit => write!(f, "O"),
            Space::Miss => write!(f, "X"),
            Space::Forfeit => write!(f, "i"),
            Space::Targeting => write!(f, "⌖"),
        }
    }
}

impl PartialEq for Space { // self defining equals function for Space struct
    fn eq(&self, other: &Self) -> bool {
        // println!("hi");
        format!("{}", self) == format!("{}", other) // if a value is just sitting there without a semicolon and is the return type it just returns it
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Coord { // coord data struct to represent coord in grid
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coord {
    pub(crate) fn new(x: i32, y: i32) -> Coord { // essentially a constructor
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

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

pub struct Game {
    hits: i32,
    misses: i32,
    shots_taken: i32,
    board: Board,
    valid_input: bool,
    game_done: bool,
}

impl Game {
    pub(crate) fn new() -> Game {
        Game{
            hits : 0,
            misses : 0,
            shots_taken: 0,
            board: Board::new(), // runs the constructor for Board
            valid_input: false,
            game_done: false,
        }
    }

    pub(crate) fn main(mut self){ //main game function
        println!("Welcome to battleships!\nIn this game you select a square to shoot\nto see if there is a ship hiding there.\nTry and sink all 5 ships with the lowest shots possible!\n");
        while !self.game_done { //main game loop
            while !self.valid_input { // check if input valid

                //shooting function
                //to be shoved into action below
                fn shoot(board : &mut Board, valid_input: &mut bool, game_done: &mut bool, hits: &mut i32, misses: &mut i32, shots_taken: &mut i32) {
                    let mut input_error = false;
                    let mut value_error = false;
                    let mut range_error = false;
                    let mut input = String::new();
                    let mut row = 11;
                    let mut col = 11;

                    println!("please enter the row and column in the format of : ROW,COLUMN");
                    stdin().read_line(&mut input).expect("Error reading input"); // read line and print error if error
                    let coords : Vec<&str> = input.split(',').collect(); // split read line by , into Vec
                    if coords.len() != 2 {
                        // check if theres 2 things split between ,
                        input_error = true;
                    } else {
                        row = match coords[0].trim().parse() { // see if splitted contents r numbers
                            Ok(num) => num,
                            Err(_) => {value_error = true; 1} // the one is there to prevent an error, it does jack (or i hope it does)
                        };
                        col = match coords[1].trim().parse() {
                            Ok(num) => num,
                            Err(_) => {value_error = true; 1}
                        };

                        if !value_error { // see if number is between 1 to 10
                            if row < 1 || row > 10 {
                                range_error = true;
                            } else {
                                row -= 1; // minus 1 bc arrays start from 0
                            }

                            if col < 1 || col > 10 {
                                range_error = true;
                            } else {
                                col -= 1;
                            }
                        }
                    }

                    if !input_error && !value_error && !range_error { // run if no errors
                        // println!("row: {}, col: {}", row, col);
                        let mut valid_confirm = false;
                        let mut confirm = false;
                        while !valid_confirm { // loop back if confirmation not in standard
                            board.grid[row][col] = Space::Targeting; // assign that tile to show a crosshair
                            board.print_board();
                            println!("You sure you want to shoot there?(Y/N)");
                            let mut confirm_input = String::new();
                            stdin().read_line(&mut confirm_input).expect("Error reading input");
                            if confirm_input.trim().to_lowercase() == "y" {
                                valid_confirm = true;
                                confirm = true;
                            } else if confirm_input.trim().to_lowercase() == "n" {
                                valid_confirm = true;
                                confirm = false;
                            } else {
                                println!("Please enter either Y or N");
                            }
                        }

                        if confirm { // run if user says yes
                            if board.check_hit(&Coord::new(row as i32, col as i32)) { // check if target tile is a hit
                                board.grid[row][col] = Space::Hit; // if hit set it as hit
                                *hits += 1; // add up hit counter
                            } else { // run if shot missed
                                board.grid[row][col] = Space::Miss;
                                *misses += 1;
                            }
                            *shots_taken += 1;
                        } else { // if user says no then reset the crosshair
                            board.grid[row][col] = Space::Unknown;
                        }

                        let win_con = &board.total_ship_length(); // if hit counter reaches total ship length the game is won
                        // let win_con = &1; // debug win con
                        if hits == win_con {
                            board.print_board();
                            println!("You win!\nYou took {} shots to win.", shots_taken);
                            *game_done = true;
                            *valid_input = true;
                        }

                    } else if input_error {
                        println!("Please enter in the format of: ROW,COLUMN");
                    } else if value_error {
                        println!("Please enter numbers for rows and columns");
                    } else if range_error {
                        println!("Please enter 0 to 9 for rows and columns");
                    } else {
                        println!("lolwtf") // should never happen lmao
                    }
                }

                // forfeit function
                // to be shoved into action below
                fn forfeit(board : &mut Board, valid_input: &mut bool, game_done: &mut bool, _hits: &mut i32, _misses: &mut i32, _shots_taken: &mut i32){
                    let mut spaces_unhit :Vec<Coord> = Vec::new();
                    for ship in &board.ships { // get the list of unhit tiles and shove it into a list
                        spaces_unhit.extend(ship.coord_list());
                    }

                    for coord in spaces_unhit {
                        // println!("{} {}", coord.x, coord.y);
                        if board.grid[coord.x as usize][coord.y as usize] == Space::Unknown { // change every tile that should be hit but isnt into an i
                            board.grid[coord.x as usize][coord.y as usize] = Space::Forfeit;
                        }
                    }

                    board.print_board();
                    println!("You lost!");
                    *game_done = true;
                    *valid_input = true;
                }

                // error function
                // to be shoved into action below
                fn error(_board : &mut Board, _valid_input: &mut bool, _game_done: &mut bool, _hits: &mut i32, _misses: &mut i32, _shots_taken: &mut i32){
                    println!("Invalid input");
                }

                self.board.print_board();
                println!("Misses: {}\tHits: {}\nShots fired: {}\nPlease select from the following:\n1. shoot\n2. forfeit", Space::Miss, Space::Hit, self.shots_taken);
                let stdin = stdin();
                let input = &mut String::new();
                let _ = stdin.read_line(input); // whenever u see a var name starting with _ it tells the compiler that this var is unused
                // action is a variable that takes in a function that takes in a changeable Board struct by reference, a changeable bool by reference
                // another changeable bool by reference, a changeable i32 by reference, another changeable i32 by reference, and changeable i32 by reference
                // that returns nothing
                let action : fn(&mut Board, &mut bool, &mut bool, &mut i32, &mut i32, &mut i32);
                match input.trim() {
                    // map in the correct function into the var according to user input
                    // essentially a switch case
                    "1" => action = shoot,
                    "2" => action = forfeit,
                    _ => action = error,
                }

                // run the selected action
                action(&mut self.board, &mut self.valid_input, &mut self.game_done, &mut self.hits, &mut self.misses, &mut self.shots_taken);
            }
        }
    }
}

