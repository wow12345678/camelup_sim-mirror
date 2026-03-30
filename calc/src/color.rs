use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Blue,
    Green,
    Orange,
    White,
    Yellow,
}

impl Color {
    pub fn all() -> [Color; 5] {
        [
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ]
    }

    pub fn as_byte(&self) -> u8 {
        let mask = 0b0000_0001;
        let index: u8 = (*self).into();
        mask << (7 - index)
    }

    pub(crate) fn try_from_byte(color_code: u8) -> Result<Self, ColorConversionError<u8>> {
        match color_code {
            0b1000_0000 => Ok(Color::Blue),
            0b0100_0000 => Ok(Color::Green),
            0b0010_0000 => Ok(Color::Orange),
            0b0001_0000 => Ok(Color::White),
            0b0000_1000 => Ok(Color::Yellow),
            _ => Err(ColorConversionError(color_code)),
        }
    }
}

#[derive(Debug)]
pub struct ColorConversionError<A>(A);

impl<A: Display> Display for ColorConversionError<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid converion from {} with value {}",
            std::any::type_name::<A>(),
            self.0
        )
    }
}

impl TryFrom<usize> for Color {
    type Error = ColorConversionError<usize>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Blue),
            1 => Ok(Color::Green),
            2 => Ok(Color::Orange),
            3 => Ok(Color::White),
            4 => Ok(Color::Yellow),
            _ => Err(ColorConversionError(value)),
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
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = ColorConversionError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Blue),
            1 => Ok(Color::Green),
            2 => Ok(Color::Orange),
            3 => Ok(Color::White),
            4 => Ok(Color::Yellow),
            _ => Err(ColorConversionError(value)),
        }
    }
}
