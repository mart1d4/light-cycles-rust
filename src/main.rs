mod board;
mod user;

use colored::Colorize;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io;

use crate::board::Board;
use crate::user::{Color, User};

const MIN_PLAYERS: u8 = 2;
const MAX_PLAYERS: u8 = 4;

fn main() {
    let mut board: Board = create_board();
    let mut users: Vec<User> = create_users();

    board.add_players(&mut users);

    while !board.is_game_ended(&users) {
        let unmut_users = users.clone();

        for user in &mut users {
            print!("\x1B[2J\x1B[1;1H");
            if user.has_lost() {
                continue;
            }

            board.print_board(&unmut_users);

            let choices = board.get_directions(user);

            if choices.0.is_empty() {
                user.set_has_lost(true);
                println!("{} has lost!", user.username());
                println!(
                    "Remaining players: {}",
                    board.remaining_players(&unmut_users, Some(user.username()))
                );
                continue;
            }

            println!(
                "{}, it's your turn to play! (Symbol {})",
                user.username(),
                user.symbol()
            );
            println!("Choose a direction to move to ({}):", choices.1);

            let mut direction = get_string_or_retry();
            while !choices.0.contains(&direction) {
                print!("\x1B[2J\x1B[1;1H");
                board.print_board(&unmut_users);
                println!(
                    "Invalid direction. Please choose a valid direction ({}):",
                    choices.1
                );
                direction = get_string_or_retry();
            }

            board.move_user(user, &direction);
        }
    }

    print!("\x1B[2J\x1B[1;1H");
    board.print_board(&users);
    println!("Game over!");
    println!(
        "The winner is: {}",
        board.remaining_players(&users, None).green()
    );
}

fn create_board() -> Board {
    print!("\x1B[2J\x1B[1;1H");
    println!("Choose the number of rows the board should have:");
    let board_size_row = get_num_between_or_retry(5, 200);

    print!("\x1B[2J\x1B[1;1H");
    println!("Choose the number of columns the board should have:");
    let board_size_column = get_num_between_or_retry(5, 200);

    let total_cells = board_size_column * board_size_row;

    print!("\x1B[2J\x1B[1;1H");
    println!("Choose the number of walls the board should have:");
    let wall_amount = get_num_between_or_retry(0, total_cells);

    print!("\x1B[2J\x1B[1;1H");
    println!("Choose a symbol to display the walls");
    let wall_char = get_char_or_retry();

    print!("\x1B[2J\x1B[1;1H");
    println!("Choose the number of boosts the board should have:");
    let boost_amount = get_num_between_or_retry(0, total_cells);

    print!("\x1B[2J\x1B[1;1H");
    println!("Choose a symbol to display the boosts");
    let boost_char = get_char_or_retry();

    Board::new(
        board_size_row,
        board_size_column,
        wall_amount,
        boost_amount,
        wall_char,
        boost_char,
    )
    .unwrap()
}

fn create_users() -> Vec<User> {
    print!("\x1B[2J\x1B[1;1H");
    println!("How many users will be playing, bots included? ({MIN_PLAYERS}-{MAX_PLAYERS})");
    let num_players = get_num_between_or_retry(2, 10);

    print!("\x1B[2J\x1B[1;1H");
    println!("How many bots will be playing? (0-{num_players})");
    let num_bots = get_num_between_or_retry(0, num_players);

    let mut users: Vec<User> = Vec::new();
    for i in 0..num_players - num_bots {
        print!("\x1B[2J\x1B[1;1H");
        println!("Enter username for player {}", i + 1);
        let mut username = get_string_or_retry();

        while is_username_taken(&username, &users) {
            print!("\x1B[2J\x1B[1;1H");
            println!("Username already taken. Please enter a different username");
            username = get_string_or_retry();
        }

        print!("\x1B[2J\x1B[1;1H");
        print_color_choices();
        let mut color = get_color(get_num_between_or_retry(1, 10) as usize);

        while is_color_taken(&color, &users) {
            print!("\x1B[2J\x1B[1;1H");
            println!("Color already taken. Please enter a different color");
            print_color_choices();
            color = get_color(get_num_between_or_retry(1, 10) as usize);
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("Enter a symbol for the player (only one character):");
        let mut symbol = get_char_or_retry();

        while is_symbol_taken(symbol, &users) {
            print!("\x1B[2J\x1B[1;1H");
            println!("Symbol already taken. Please enter a different symbol");
            symbol = get_char_or_retry();
        }

        users.push(User::new(username, color, symbol, false));
    }

    for i in 0..num_bots {
        let mut random_color = get_random_color();
        while is_color_taken(&random_color, &users) {
            random_color = get_random_color();
        }

        let mut random_symbol = get_random_char();
        while is_symbol_taken(random_symbol, &users) {
            random_symbol = get_random_char();
        }

        let bot = User::new(format!("Bot {}", i + 1), random_color, random_symbol, true);
        users.push(bot);
    }

    users
}

fn is_username_taken(username: &str, users: &Vec<User>) -> bool {
    for user in users {
        if user.username() == username {
            return true;
        }
    }
    false
}

fn is_color_taken(color: &Color, users: &Vec<User>) -> bool {
    for user in users {
        if user.color() == *color {
            return true;
        }
    }
    false
}

fn is_symbol_taken(symbol: char, users: &Vec<User>) -> bool {
    for user in users {
        if user.symbol() == symbol {
            return true;
        }
    }
    false
}

fn get_char_or_retry() -> char {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string().chars().next().unwrap()
}

fn get_string_or_retry() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_num_between_or_retry(min: u32, max: u32) -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number between {} and {}", min, max);
            get_num_between_or_retry(min, max)
        }
    };

    if input < min || input > max {
        println!("Please enter a number between {} and {}", min, max);
        get_num_between_or_retry(min, max)
    } else {
        input
    }
}

fn print_color_choices() {
    println!(
        "Enter a number for the color of the player:\n\
        1. {}      5. {}\n\
        2. {}      6. {}\n\
        3. {}       7. {}\n\
        4. {}        8. {}",
        "Black".black(),
        "Green".green(),
        "White".white(),
        "Yellow".yellow(),
        "Blue".blue(),
        "Magenta".magenta(),
        "Red".red(),
        "Cyan".cyan(),
    );
}

fn get_random_color() -> Color {
    let random = rand::thread_rng().gen_range(1..9);
    get_color(random)
}

fn get_color(matcher: usize) -> Color {
    match matcher {
        1 => Color::Black,
        2 => Color::White,
        3 => Color::Blue,
        4 => Color::Red,
        5 => Color::Green,
        6 => Color::Yellow,
        7 => Color::Magenta,
        8 => Color::Cyan,
        _ => panic!("Invalid color number"),
    }
}

fn get_random_char() -> char {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .next()
        .unwrap()
}
