use crate::user::{Color, Place, User};
use rand::Rng;

use colored::Colorize;
const MAX_SIZE: u8 = 30;

pub struct Board {
    board: Vec<Vec<char>>,
    rows: u32,
    columns: u32,
    wall_char: char,
    boost_char: char,
    boost_count: u32,
}

impl Board {
    pub fn new(
        rows: u32,
        columns: u32,
        walls: u32,
        boosts: u32,
        wall_char: char,
        boost_char: char,
    ) -> Result<Board, String> {
        if boosts + walls + 20 >= rows * columns {
            return Err(format!(
                "\
                Board isn't big enough to contain all of these objects.\
                Board has {} cells but you want {} items in the board
            ",
                rows * columns,
                boosts + walls,
            ));
        } else if rows > u32::try_from(MAX_SIZE).unwrap()
            || columns > u32::try_from(MAX_SIZE).unwrap()
        {
            return Err(format!("Board is too big. Choose a lower size."));
        }

        let mut board =
            vec![vec![' '; usize::try_from(columns).unwrap()]; usize::try_from(rows).unwrap()];

        for _ in 0..boosts {
            let mut r = get_random_usize(0, rows)?;
            let mut c = get_random_usize(0, columns)?;

            while board[r][c] == boost_char {
                r = get_random_usize(0, rows)?;
                c = get_random_usize(0, columns)?;
            }

            board[r][c] = boost_char;
        }

        for _ in 0..walls {
            let mut r = get_random_usize(0, rows)?;
            let mut c = get_random_usize(0, columns)?;

            // Since boosts have already been added, need to check for them too
            while board[r][c] == wall_char || board[r][c] == boost_char {
                r = get_random_usize(0, rows)?;
                c = get_random_usize(0, columns)?;
            }

            board[r][c] = wall_char;
        }

        Ok(Board {
            board,
            rows,
            columns,
            boost_char,
            wall_char,
            boost_count: boosts,
        })
    }

    pub fn add_players(&mut self, users: &mut Vec<User>) {
        for (i, user) in users.iter_mut().enumerate() {
            match i {
                0 => {
                    self.board[0][0] = user.symbol();
                }
                1 => {
                    self.board[0][(self.columns - 1) as usize] = user.symbol();
                    user.set_place(Place {
                        row: 0,
                        col: (self.columns - 1) as usize,
                    })
                }
                2 => {
                    self.board[(self.rows - 1) as usize][0] = user.symbol();
                    user.set_place(Place {
                        row: (self.rows - 1) as usize,
                        col: 0,
                    })
                }
                3 => {
                    self.board[(self.rows - 1) as usize][(self.columns - 1) as usize] =
                        user.symbol();
                    user.set_place(Place {
                        row: (self.rows - 1) as usize,
                        col: (self.columns - 1) as usize,
                    })
                }
                _ => panic!("Ohh nooo too many players"),
            }
        }
    }

    pub fn print_board(&self, users: &Vec<User>) {
        // Print top horizontal line
        print!("┌");
        for _ in 0..self.board[0].len() - 1 {
            print!("───┬");
        }
        println!("───┐");

        // Print rows with contents
        let mut i = 0;
        let mut j = 0;
        for row in &self.board {
            // Print contents of the row
            print!("│");
            for cell in row {
                let (is_user, symbol) = symbol_at_place(users, i, j);

                if is_user {
                    print!(" {} ", symbol);
                } else {
                    match get_color_from_char(cell, users) {
                        Some(x) => match x {
                            Color::Black => {
                                print!("{}", format!("   ").to_string().on_black())
                            }
                            Color::White => {
                                print!("{}", format!("   ").to_string().on_white())
                            }
                            Color::Red => print!("{}", format!("   ").to_string().on_red()),
                            Color::Blue => print!("{}", format!("   ").to_string().on_blue()),
                            Color::Green => {
                                print!("{}", format!("   ").to_string().on_green())
                            }
                            Color::Yellow => {
                                print!("{}", format!("   ").to_string().on_yellow())
                            }
                            Color::Magenta => {
                                print!("{}", format!("   ").to_string().on_magenta())
                            }
                            Color::Cyan => print!("{}", format!("   ").to_string().on_cyan()),
                        },
                        None => print!(" {} ", cell),
                    }
                }

                print!("│");
                j += 1;
            }
            println!();
            j = 0;

            // Print horizontal line between rows
            if i + 1 != self.board.len() {
                print!("├───");
                for _ in 0..self.board[0].len() - 1 {
                    print!("┼───");
                }
                println!("┤");
            }

            i += 1;
        }

        // Print bottom horizontal line
        print!("└");
        for _ in 0..self.board[0].len() - 1 {
            print!("───┴");
        }
        println!("───┘");
    }

    fn is_valid_cell(&self, row: i32, column: i32) -> bool {
        row >= 0
            && row < i32::try_from(self.rows).unwrap()
            && column >= 0
            && column < i32::try_from(self.columns).unwrap()
            && (self.board[row as usize][column as usize] == ' '
                || self.board[row as usize][column as usize] == self.boost_char)
    }

    pub fn get_directions(&self, user: &User) -> (Vec<String>, String) {
        println!(
            "Debug: currently calculating dirs for user: {} with place: x: {}, y: {}",
            user.username(),
            user.place().row,
            user.place().col
        );

        let mut directions = vec![];
        let mut display = String::new();
        let place = user.place();

        if self.is_valid_cell(place.row as i32 - 1, place.col as i32) {
            directions.push("Up".to_string());
            display.push_str("Up ");
        }

        if self.is_valid_cell(place.row as i32 + 1, place.col as i32) {
            directions.push("Down".to_string());
            display.push_str("Down ");
        }

        if self.is_valid_cell(place.row as i32, place.col as i32 - 1) {
            directions.push("Left".to_string());
            display.push_str("Left ");
        }

        if self.is_valid_cell(place.row as i32, place.col as i32 + 1) {
            directions.push("Right".to_string());
            display.push_str("Right ");
        }

        let mut chars = display.chars();
        chars.next_back();

        display = chars.as_str().to_string();

        (directions, display)
    }

    pub fn move_user(&mut self, user: &mut User, direction: &str) {
        let place = user.place();
        let mut new_place = place.clone();

        match direction {
            "Up" => new_place.row -= 1,
            "Down" => new_place.row += 1,
            "Left" => new_place.col -= 1,
            "Right" => new_place.col += 1,
            _ => panic!("Invalid direction"),
        }

        if self.board[new_place.row][new_place.col] == self.boost_char {
            self.boost_count -= 1;
            user.add_boost();
        }

        if self.board[new_place.row][new_place.col] == self.wall_char {
            user.remove_boost();
        }

        self.board[new_place.row][new_place.col] = user.symbol();
        user.set_place(new_place);
    }

    pub fn is_game_ended(&self, users: &Vec<User>) -> bool {
        let mut count = 0;
        for user in users {
            if user.has_lost() {
                count += 1;
            }
        }

        count == users.len() - 1
    }

    pub fn remaining_players(&self, users: &Vec<User>, username: Option<String>) -> String {
        let mut remaining = String::from("");
        for user in users {
            if !user.has_lost() && Some(user.username()) != username {
                remaining.push_str(&user.username());
                remaining.push_str(", ");
            }
        }

        remaining.pop();
        remaining.pop();
        remaining
    }
}

fn get_random_usize(min: u32, max: u32) -> Result<usize, String> {
    let random_num = rand::thread_rng().gen_range(min..max);
    let num_usize = usize::try_from(random_num).unwrap();
    Ok(num_usize)
}

fn get_color_from_char(char: &char, users: &Vec<User>) -> Option<Color> {
    for user in users {
        if *char == user.symbol() {
            return Some(user.color());
        }
    }

    None
}

// Checks whether one of the player's place is at the given place,
// if so, return his symbol, otherwise None
fn symbol_at_place(users: &Vec<User>, row: usize, col: usize) -> (bool, char) {
    for user in users {
        if user.place().row == row && user.place().col == col {
            return (true, user.symbol());
        }
    }

    (false, ' ')
}
