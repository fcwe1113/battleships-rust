use std::cmp::PartialEq;
use std::fmt;
use std::io::stdin;
use crate::board::Board;

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

                fn shoot(board : &mut Board, valid_input: &mut bool, game_done: &mut bool){
                    println!("What do you want to do?");
                }

                fn forfeit(board : &mut Board, valid_input: &mut bool, game_done: &mut bool){
                    let mut spaces_unhit :Vec<Coord> = Vec::new();
                    for ship in &board.ships {
                        spaces_unhit.extend(ship.coord_list());
                    }

                    for coord in spaces_unhit {
                        // println!("{} {}", coord.x, coord.y);
                        board.grid[coord.x as usize][coord.y as usize] = Space::Forfeit;
                    }

                    board.print_board();
                    println!("You lost!");
                    *game_done = true;
                    *valid_input = true;
                }

                fn error(board : &mut Board, valid_input: &mut bool, game_done: &mut bool){
                    println!("What do you want to error?");
                }

                self.board.print_board();
                println!("Misses: {}\tHits: {}\nShots fired: {}\nPlease select from the following:\n1. shoot\n2. forfeit", Space::Miss, Space::Hit, self.shots_taken);
                let mut stdin = stdin();
                let mut input = &mut String::new();
                let _ = stdin.read_line(input);
                let action : fn(&mut Board, &mut bool, &mut bool);
                match input.trim() {
                    "1" => action = shoot,
                    "2" => action = forfeit,
                    _ => action = error,
                }

                action(&mut self.board, &mut self.valid_input, &mut self.game_done);
            }
        }
    }
}

