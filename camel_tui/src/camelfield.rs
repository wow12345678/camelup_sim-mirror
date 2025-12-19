use std::{cmp::max, fmt::Display};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CamelColor {
    Blue,
    Green,
    Yellow,
    Orange,
    White,
}

impl Display for CamelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CamelColor::Blue => write!(f, "Blue"),
            CamelColor::Green => write!(f, "Green"),
            CamelColor::Yellow => write!(f, "Yellow"),
            CamelColor::Orange => write!(f, "Orange"),
            CamelColor::White => write!(f, "White"),
        }
    }
}

impl From<CamelColor> for usize {
    fn from(value: CamelColor) -> Self {
        match value {
            CamelColor::Blue => 0,
            CamelColor::Green => 1,
            CamelColor::Yellow => 2,
            CamelColor::Orange => 3,
            CamelColor::White => 4,
        }
    }
}

impl From<usize> for CamelColor {
    fn from(value: usize) -> Self {
        match value {
            0 => CamelColor::Blue,
            1 => CamelColor::Green,
            2 => CamelColor::Yellow,
            3 => CamelColor::Orange,
            4 => CamelColor::White,
            _ => panic!("Not a valid value for Color conversion"),
        }
    }
}

impl CamelColor {
    pub fn all() -> [CamelColor; 5] {
        [
            CamelColor::Blue,
            CamelColor::Green,
            CamelColor::Yellow,
            CamelColor::Orange,
            CamelColor::White,
        ]
    }

    pub fn from_char_to_usize(c: char) -> usize {
        match c {
            'b' => 0,
            'g' => 1,
            'y' => 2,
            'o' => 3,
            'w' => 4,
            _ => panic!("invalid character"),
        }
    }

    pub const fn text_color(self) -> Color {
        match self {
            CamelColor::Blue => Color::Rgb(255, 255, 0),
            CamelColor::Green => Color::Rgb(0, 255, 0),
            CamelColor::Yellow => Color::Rgb(255, 255, 0),
            CamelColor::Orange => Color::Rgb(255, 165, 0),
            CamelColor::White => Color::Rgb(0, 0, 0),
        }
    }

    pub const fn to_color(self) -> Color {
        match self {
            CamelColor::Blue => Color::Rgb(52, 18, 237),
            CamelColor::Green => Color::Rgb(0, 255, 0),
            CamelColor::Yellow => Color::Rgb(255, 255, 0),
            CamelColor::Orange => Color::Rgb(255, 165, 0),
            CamelColor::White => Color::Rgb(255, 255, 255),
        }
    }
}

const CAMEL_HEIGHT: usize = 15;
const CAMEL_WIDTH: usize = 19;

#[rustfmt::skip]
const CAMEL_PATTERN_FACING_RIGHT: [[bool; CAMEL_WIDTH]; CAMEL_HEIGHT] = [
    [false, false, false, false, false, false, false,false, false, false, false, false, false, false, false, true, true,  false, false ],
    [false, false, false, false, false, false, false,false, false, false, false, false, false, false, true,  true, true,  true,  true  ],
    [false, false, false, false, false, false, true, true,  true,  true,  false, false, false, false, true,  true, true,  true,  true  ],
    [false, false, false, false, false, true,  true, true,  true,  true,  true,  false, false, false, true,  true, true,  false, false ],
    [false, false, false, false, true,  true,  true, true,  true,  true,  true,  true,  false, false, true,  true, true,  false, false ],
    [false, false, false, true,  true,  true,  true, true,  true,  true,  true,  true,  true,  false, true,  true, true,  false, false ],
    [false, true,  true,  true,  true,  true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true, true,  false, false ],
    [true , false, true,  true,  true,  true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true, true,  false, false ],
    [true , false, true,  true,  true,  true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true, true,  false, false ],
    [true , false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [true , false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  true,  false, true, true,  false, false, false, false, true,  true,  false, true, true,  false, false ],
];

#[rustfmt::skip]
const CAMEL_PATTERN_FACING_LEFT: [[bool; CAMEL_WIDTH]; CAMEL_HEIGHT] = [
    [false, false, true,  true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [true,  true,  true,  true, true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [true,  true,  true,  true, true,  false, false, false, false, true,  true,  true,  true,  false, false, false, false, false, false],
    [false, false, true,  true, true,  false, false, false, true,  true,  true,  true,  true,  true,  false, false, false, false, false],
    [false, false, true,  true, true,  false, false, true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false],
    [false, false, true,  true, true,  false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false],
    [false, false, true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false],
    [false, false, true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true],
    [false, false, true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true],
    [false, false, false, true, false, false, true,  false, false, false, false, false, true,  false, false, true,  false, false, true],
    [false, false, false, true, false, false, true,  false, false, false, false, false, true,  false, false, true,  false, false, true],
    [false, false, false, true, false, false, true,  false, false, false, false, false, true,  false, false, true,  false, false, false],
    [false, false, false, true, false, false, true,  false, false, false, false, false, true,  false, false, true,  false, false, false],
    [false, false, false, true, false, false, true,  false, false, false, false, false, true,  false, false, true,  false, false, false],
    [false, false, true,  true, false, true,  true,  false, false, false, false, true,  true,  false, true,  true,  false, false, false],
];

/// camels are ordered from bottom to top
/// index is the index in the representation in GameField
/// board_index is the index that is rendered for the field
/// board representation:
///  ______________
/// |14|15| 0| 1| 2|
/// |13|‾‾‾‾‾‾‾‾| 3|
/// |12|        | 4|
/// |11|________| 5|
/// |10| 9| 8| 7| 6|
///  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾
#[derive(Debug, Default)]
pub struct CamelField {
    pub camels: Vec<CamelColor>,
    board_index: usize,
    index: usize,
    pub selected: bool,
}

impl CamelField {
    fn new(camels: Vec<CamelColor>, index: usize, board_index: usize) -> Self {
        Self {
            camels,
            board_index,
            index,
            selected: false,
        }
    }

    fn render_camels(&self, area: Rect, buf: &mut Buffer) {
        if self.camels.is_empty() {
            return;
        }

        let x_offset = area.x + area.width / 4;
        let field_bottom = area.y + area.height - 1;

        // Convert pixel height to character height
        let camel_height_chars = CAMEL_HEIGHT.div_ceil(2) as u16;
        let total_camels = self.camels.len();
        // borders
        let available_height = area.height - 2;

        let camel_stacking_offset = if total_camels == 0 {
            0
        } else {
            max(1, available_height / (total_camels as u16 + 1))
        };

        for camel_idx in 0..self.camels.len() {
            let col = self.camels[camel_idx].to_color();
            let mut y = 0;

            let camel_pattern = if (0..8).contains(&self.index) {
                CAMEL_PATTERN_FACING_RIGHT
            } else {
                CAMEL_PATTERN_FACING_LEFT
            };

            // TODO: think about good camel rendering: either camel_idx or stack_position or
            // something else
            let stack_position = total_camels - camel_idx - 1;
            let y_base =
                field_bottom - camel_height_chars - stack_position as u16 * camel_stacking_offset;

            while y < CAMEL_HEIGHT {
                // Process two vertical pixels at once except for odd remainder
                if y + 1 < CAMEL_HEIGHT {
                    for x in 0..CAMEL_WIDTH {
                        let char_x = x as u16 + x_offset;
                        let char_y = y_base + (y / 2) as u16;

                        if char_x < buf.area.right()
                            && char_y < buf.area.bottom()
                            && char_y >= buf.area.top()
                        {
                            match (camel_pattern[y][x], camel_pattern[y + 1][x]) {
                                (true, true) => {
                                    buf[(char_x, char_y)].set_char('▀').set_fg(col).set_bg(col);
                                }
                                (true, false) => {
                                    buf[(char_x, char_y)].set_char('▀').set_fg(col);
                                }
                                (false, true) => {
                                    buf[(char_x, char_y)].set_char('▄').set_fg(col);
                                }
                                (false, false) => {
                                    // Both pixels empty, skip
                                }
                            }
                        }
                    }
                    y += 2; // Increment by 2 since we processed 2 rows
                } else {
                    // Handle odd remainder row
                    for x in 0..CAMEL_WIDTH {
                        if camel_pattern[y][x] {
                            let char_x = x as u16 + x_offset;
                            let char_y = y_base + (y / 2) as u16;

                            if char_x < buf.area.right()
                                && char_y < buf.area.bottom()
                                && char_y >= buf.area.top()
                            {
                                buf[(char_x, char_y)].set_char('▀').set_fg(col);
                            }
                        }
                    }
                    y += 1;
                }
            }
        }
    }
}

impl Widget for &CamelField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border_color = if self.selected {
            Color::LightBlue
        } else {
            Color::White
        };

        if cfg!(debug_assertions) {
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_top(format!(
                    "Field {}, area:{},{},{},{}",
                    self.board_index, area.x, area.y, area.width, area.height
                ))
                .render(area, buf);
        } else {
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_top(format!("Field {}", self.board_index))
                .render(area, buf);
        }

        self.render_camels(area, buf);
    }
}

