pub enum Color {
    Black,
    Red,
    Yellow,
}

pub struct User {
    username: String,
    color: String,
    symbol: char,

    is_bot: bool,
}

impl User {
    pub fn new(username: &str, color: Color, symbol: char, is_bot: bool) -> Result<User, &'static str> {
        let color_string = get_color(color).unwrap();

        Ok(User {
            username: username.to_string(),
            color: color_string,
            symbol,
            is_bot,
        })
    }

}

fn get_color(color: Color) -> Result<String, &'static str> {
    match color {
        Color::Red => Ok("#FF0000".to_string()),
        _ => Err("Wrong color")
    }
}
