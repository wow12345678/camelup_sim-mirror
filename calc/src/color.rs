#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Blue,
    Green,
    Orange,
    White,
    Yellow,
    None,
}

impl Color {
    pub fn as_byte(&self) -> u8 {
        let mask = 0b0000_0001;
        let index: u8 = (*self).into();
        mask << (7 - index)
    }

    pub(crate) fn from_byte(color_code: u8) -> Self {
        match color_code {
            0b1000_0000 => Color::Blue,
            0b0100_0000 => Color::Green,
            0b0010_0000 => Color::Orange,
            0b0001_0000 => Color::White,
            0b0000_1000 => Color::Yellow,
            _ => Color::None,
        }
    }
}

impl From<usize> for Color {
    fn from(value: usize) -> Self {
        match value {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Orange,
            3 => Color::White,
            4 => Color::Yellow,
            _ => Color::None,
        }
    }
}

impl From<Color> for usize {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => 0,
            Color::Green => 1,
            Color::Orange => 2,
            Color::White => 3,
            Color::Yellow => 4,
            Color::None => 5,
        }
    }
}

impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => 0,
            Color::Green => 1,
            Color::Orange => 2,
            Color::White => 3,
            Color::Yellow => 4,
            Color::None => 5,
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Orange,
            3 => Color::White,
            4 => Color::Yellow,
            _ => Color::None,
        }
    }
}
