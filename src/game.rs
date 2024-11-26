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
impl fmt::Display for Space {
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

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        // println!("hi");
        format!("{}", self) == format!("{}", other)
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

impl fmt::Display for Direction {
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
    target: Vec<Coord>,
}



impl Game {
    pub(crate) fn new() -> Game {
        Game{
            hits : 0,
            misses : 0,
            shots_taken: 0,
            board: Board::new(),
            valid_input: false,
            game_done: false,
            target: Vec::new(),
        }
    }


    fn get_targets(board : &Board){
        for i in 0..board.ships.len() {

        }
    }

    pub(crate) fn main(mut self){
        println!("Welcome to battleships!\nIn this game you select a square to shoot\nto see if there is a ship hiding there.\nTry and sink all 5 ships with the lowest shots possible!\n");
        while !self.game_done {
            while !self.valid_input {

                fn shoot(board : &mut Board, valid_input: &mut bool, game_done: &mut bool, hits: &mut i32, misses: &mut i32, shots_taken: &mut i32) {
                    let mut input_error = false;
                    let mut value_error = false;
                    let mut range_error = false;
                    let mut input = String::new();
                    let mut row = 11;
                    let mut col = 11;

                    println!("please enter the row and column in the format of : ROW,COLUMN");
                    stdin().read_line(&mut input).expect("Error reading input");
                    let coords : Vec<&str> = input.split(',').collect();
                    if coords.len() != 2 {
                        input_error = true;
                    } else {
                        row = match coords[0].trim().parse() {
                            Ok(num) => num,
                            Err(_) => {value_error = true; 1} // the one is there to prevent an error, it does jack (or i hope it does)
                        };
                        col = match coords[1].trim().parse() {
                            Ok(num) => num,
                            Err(_) => {value_error = true; 1}
                        };

                        if !value_error {
                            if row < 1 || row > 10 {
                                range_error = true;
                            } else {
                                row -= 1;
                            }

                            if col < 1 || col > 10 {
                                range_error = true;
                            } else {
                                col -= 1;
                            }
                        }
                    }

                    if !input_error && !value_error && !range_error {
                        // println!("row: {}, col: {}", row, col);
                        let mut valid_confirm = false;
                        let mut confirm = false;
                        while !valid_confirm {
                            board.grid[row][col] = Space::Targeting;
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

                        if confirm {
                            if board.check_hit(&Coord::new(row as i32, col as i32)) {
                                board.grid[row][col] = Space::Hit;
                                *hits += 1;
                            } else {
                                board.grid[row][col] = Space::Miss;
                                *misses += 1;
                            }
                            *shots_taken += 1;
                        } else {
                            board.grid[row][col] = Space::Unknown;
                        }

                        let win_con = &board.total_ship_length();
                        // let win_con = &1;
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

                fn forfeit(board : &mut Board, valid_input: &mut bool, game_done: &mut bool, hits: &mut i32, misses: &mut i32, shots_taken: &mut i32){
                    let mut spaces_unhit :Vec<Coord> = Vec::new();
                    for ship in &board.ships {
                        spaces_unhit.extend(ship.coord_list());
                    }

                    for coord in spaces_unhit {
                        // println!("{} {}", coord.x, coord.y);
                        if board.grid[coord.x as usize][coord.y as usize] == Space::Unknown {
                            board.grid[coord.x as usize][coord.y as usize] = Space::Forfeit;
                        }
                    }

                    board.print_board();
                    println!("You lost!");
                    *game_done = true;
                    *valid_input = true;
                }

                fn error(board : &mut Board, valid_input: &mut bool, game_done: &mut bool, hits: &mut i32, misses: &mut i32, shots_taken: &mut i32){
                    println!("What do you want to error?");
                }

                self.board.print_board();
                println!("Misses: {}\tHits: {}\nShots fired: {}\nPlease select from the following:\n1. shoot\n2. forfeit", Space::Miss, Space::Hit, self.shots_taken);
                let mut stdin = stdin();
                let mut input = &mut String::new();
                let _ = stdin.read_line(input);
                let action : fn(&mut Board, &mut bool, &mut bool, &mut i32, &mut i32, &mut i32);
                match input.trim() {
                    "1" => action = shoot,
                    "2" => action = forfeit,
                    _ => action = error,
                }

                action(&mut self.board, &mut self.valid_input, &mut self.game_done, &mut self.hits, &mut self.misses, &mut self.shots_taken);
            }
        }
    }
}

