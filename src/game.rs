use std::cmp::PartialEq;
use std::fmt;
use std::io::{stdin, Stdin};
use crate::board::Board;

#[derive(Copy, Clone, PartialEq, PartialOrd)] // enables the enum to be copied and cloned
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
        let mut stdin = stdin();
        println!("Welcome to battleships!\nIn this game you select a square to shoot\nto see if there is a ship hiding there.\nTry and sink all 5 ships with the lowest shots possible!\n");
        while !self.game_done {
            while !self.valid_input {

                fn shoot(stdin: &mut Stdin, game: &mut Game){
                    println!("Enter coordinate in form x,y:");
                    let mut coord_str = String::new();
                    if stdin.read_line(&mut coord_str).is_err() {
                        println!("Could not read from stdin!");
                        game.valid_input = false;
                        return;
                    }   
                    let parts = coord_str.trim().split(',').filter_map(|part| part.parse::<i32>().ok()).collect::<Vec<i32>>();
                    if parts.len() != 2 {
                        println!("{} was not a valid format!", coord_str);
                        game.valid_input = false;
                        return;
                    }
                    let coord = Coord::new(parts[0] - 1, parts[1] - 1);
                    if game.board.grid[coord.x as usize][coord.y as usize] == Space::Unknown {
                        let mut val = Space::Miss;
                        for ship in &game.board.ships {
                            if ship.is_collide(coord) {
                                val = Space::Hit;
                                break;
                            }
                        }
                        match val {
                            Space::Unknown => {},
                            Space::Hit => { 
                                game.board.grid[coord.x as usize][coord.y as usize] = Space::Hit;
                                game.hits += 1;
                                println!("Shot Hit!");
                            },
                            Space::Miss => { 
                                game.board.grid[coord.x as usize][coord.y as usize] = Space::Miss;
                                game.misses += 1;
                                println!("Shot Missed!");
                            },
                            Space::Forfeit => {}
                        }
                    }
                }

                fn forfeit(_stdin: &mut Stdin, game: &mut Game){
                    let mut spaces_unhit :Vec<Coord> = Vec::new();
                    for ship in &game.board.ships {
                        spaces_unhit.extend(ship.coord_list());
                    }

                    for coord in spaces_unhit {
                        // println!("{} {}", coord.x, coord.y);
                        game.board.grid[coord.x as usize][coord.y as usize] = Space::Forfeit;
                    }

                    game.board.print_board();
                    println!("You lost!");
                    game.game_done = true;
                    game.valid_input = true;
                }

                fn error(_stdin: &mut Stdin, game: &mut Game){
                    println!("What do you want to error?");
                }

                fn peek(_stdin: &mut Stdin, game: &mut Game) {
                    let mut buffer = String::from("-----------------------------------------\n");
                    for y in 0..10 {
                        for x in 0..10 {
                            let mut ch = "|   ";
                            for ship in &game.board.ships {
                                if ship.is_collide(Coord { x, y }) {
                                    ch = "| x ";
                                }
                            }
                            buffer += ch;
                        }
                        buffer += "|\n";
                        buffer += "-----------------------------------------\n"
                    }
                    println!("Board:\n{}", buffer);
                }

                self.board.print_board();
                println!("Misses: {}\tHits: {}\nShots fired: {}\nPlease select from the following:\n1. shoot\n2. forfeit", self.misses, self.hits, self.shots_taken);
                let mut input = String::new();
                let _ = stdin.read_line(&mut input);
                let action : fn(&mut Stdin, &mut Game);
                match input.trim() {
                    "1" => action = shoot,
                    "2" => action = forfeit,
                    "help pls" => action = peek,
                    _ => action = error,
                }

                action(&mut stdin, &mut self);
            }
        }
    }
}


