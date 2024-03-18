mod user;
use core::panic;

use crate::user::{User, Color};

fn main() {
    create_user();
}

fn create_user() {
    let _user = match User::new("mart1d4", Color::Black, 'X', false) {
        Ok(u) => u,
        Err(err) => panic!("Oh no! {err}"),
    };

    println!("Seems to be working fine")
}
