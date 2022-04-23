use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerSymbol {
    Cross,
    Circle,
}

impl fmt::Display for PlayerSymbol{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PlayerSymbol::Cross => write!(f, "  X  "),
            PlayerSymbol::Circle => write!(f, "  O  "),
        }
    }
}

pub struct Players{
    pub symbol: PlayerSymbol,
}

impl Players {

    pub fn new() -> Players {
        let x = rand::random::<f64>()*10 as f64;
        let x_floor = x.floor();
        Players { symbol: Players::set_player_symbol(x_floor % 2.0 == 0.0) }
    }

    pub fn set_player_symbol(condition: bool) -> PlayerSymbol {
        if condition {
            PlayerSymbol::Cross
        } else {
            PlayerSymbol::Circle
        }
    }
}