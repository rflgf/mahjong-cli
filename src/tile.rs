use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Dragon {
    Green,
    Red,
    White,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Simple {
    Man(u8),
    Pin(u8),
    Sou(u8),
}
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Tile {
    Dragon(Dragon),
    Simple(Simple),
    Wind(Wind),

    None,
}

impl Tile {
    fn indicate_dora(self) -> Tile {
        match self {
            Tile::Dragon(Dragon::Green) => Tile::Dragon(Dragon::Red),
            Tile::Dragon(Dragon::Red) => Tile::Dragon(Dragon::White),
            Tile::Dragon(Dragon::White) => Tile::Dragon(Dragon::Green),

            Tile::Simple(Simple::Man(number)) => match number {
                1..=8 => Tile::Simple(Simple::Man(number + 1)),
                9 => Tile::Simple(Simple::Man(1)),
                _ => Tile::None,
            },

            Tile::Simple(Simple::Pin(number)) => match number {
                1..=8 => Tile::Simple(Simple::Pin(number + 1)),
                9 => Tile::Simple(Simple::Pin(1)),
                _ => Tile::None,
            },

            Tile::Simple(Simple::Sou(number)) => match number {
                1..=8 => Tile::Simple(Simple::Sou(number + 1)),
                9 => Tile::Simple(Simple::Sou(1)),
                _ => Tile::None,
            },

            Tile::Wind(Wind::East) => Tile::Wind(Wind::South),
            Tile::Wind(Wind::South) => Tile::Wind(Wind::West),
            Tile::Wind(Wind::West) => Tile::Wind(Wind::North),
            Tile::Wind(Wind::North) => Tile::Wind(Wind::East),

            Tile::None => Tile::None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Dragon(Dragon::Green) => write!(f, "GD"),
            Tile::Dragon(Dragon::Red) => write!(f, "RD"),
            Tile::Dragon(Dragon::White) => write!(f, "WD"),

            Tile::Simple(Simple::Man(number)) => write!(f, "M{}", number),
            Tile::Simple(Simple::Pin(number)) => write!(f, "P{}", number),
            Tile::Simple(Simple::Sou(number)) => write!(f, "S{}", number),

            Tile::Wind(Wind::East) => write!(f, "EW"),
            Tile::Wind(Wind::South) => write!(f, "SW"),
            Tile::Wind(Wind::West) => write!(f, "WW"),
            Tile::Wind(Wind::North) => write!(f, "NW"),

            Tile::None => write!(f, "!!"),
        }
    }
}
