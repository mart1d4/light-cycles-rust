#[derive(PartialEq, Clone)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

#[derive(Clone)]
pub struct Place {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct User {
    username: String,
    color: Color,
    symbol: char,

    place: Place,
    boosts: u8,
    has_lost: bool,

    is_bot: bool,
}

impl User {
    pub fn new(username: String, color: Color, symbol: char, is_bot: bool) -> User {
        User {
            username,
            color,
            symbol,
            place: Place { row: 0, col: 0 },
            boosts: 0,
            has_lost: false,
            is_bot,
        }
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn is_bot(&self) -> bool {
        self.is_bot
    }

    pub fn place(&self) -> Place {
        self.place.clone()
    }

    pub fn set_place(&mut self, place: Place) {
        self.place = place;
    }

    pub fn has_lost(&self) -> bool {
        self.has_lost
    }

    pub fn set_has_lost(&mut self, has_lost: bool) {
        self.has_lost = has_lost;
    }

    pub fn boosts(&self) -> u8 {
        self.boosts
    }

    pub fn add_boost(&mut self) {
        self.boosts += 1;
    }

    pub fn remove_boost(&mut self) {
        self.boosts -= 1;
    }
}
