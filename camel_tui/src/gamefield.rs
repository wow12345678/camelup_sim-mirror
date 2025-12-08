use std::{array, cmp::max, fmt::Display, slice::Iter};

use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Widget},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CamelColor {
    Red,
    Green,
    Yellow,
    Orange,
    White,
}

impl Display for CamelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CamelColor::Red => write!(f, "Red"),
            CamelColor::Green => write!(f, "Green"),
            CamelColor::Yellow => write!(f, "Yellow"),
            CamelColor::Orange => write!(f, "Orange"),
            CamelColor::White => write!(f, "White"),
        }
    }
}

impl CamelColor {
    pub fn all() -> [CamelColor; 5] {
        [
            CamelColor::Red,
            CamelColor::Green,
            CamelColor::Yellow,
            CamelColor::Orange,
            CamelColor::White,
        ]
    }

    pub fn from_char_to_int(c: char) -> usize {
        match c {
            'r' => 0,
            'g' => 1,
            'y' => 2,
            'o' => 3,
            'w' => 4,
            _ => panic!("invalid character"),
        }
    }

    pub const fn text_color(self) -> Color {
        match self {
            CamelColor::Red => Color::Rgb(255, 255, 255),
            CamelColor::Green => Color::Rgb(0, 255, 0),
            CamelColor::Yellow => Color::Rgb(255, 255, 0),
            CamelColor::Orange => Color::Rgb(255, 165, 0),
            CamelColor::White => Color::Rgb(0, 0, 0),
        }
    }

    pub const fn to_color(self) -> Color {
        match self {
            CamelColor::Red => Color::Rgb(255, 0, 0),
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
    selected: bool,
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
                .title_top(format!("Field {}", self.board_index))
                .render(area, buf);
        }

        self.render_camels(area, buf);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Focused(usize),
    Unfocused(usize),
}

#[derive(Debug)]
pub struct GameField {
    pub fields: [CamelField; 16],
    selected: State,
}

impl GameField {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let initial_index = 0;

        // [idx:14,idx:15,idx:0,...,idx:13]
        let fields = array::from_fn(|i| {
            if i < 2 {
                CamelField::new(Vec::new(), i, i + 14)
            } else {
                CamelField::new(Vec::new(), i, i - 2)
            }
        });

        let init_state = State::Unfocused(initial_index);

        Self {
            fields,
            selected: init_state,
        }
    }

    pub fn focus(&mut self) {
        if let State::Unfocused(curr) = self.selected {
            self.selected = State::Focused(curr);
            self.fields[curr].selected = true;
        } else {
            panic!("The GameField should be unfocused if the method is called")
        }
    }

    pub fn unfocus(&mut self) {
        if let State::Focused(curr) = self.selected {
            self.selected = State::Unfocused(curr);
            self.fields[curr].selected = false;
        } else {
            panic!("The GameField should be focused if the method is called")
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode) {
        match (key, self.selected) {
            (KeyCode::Enter, _) => todo!(),
            (KeyCode::Right | KeyCode::Char('l'), State::Focused(0..4)) => {
                self.change_selection_rel(1)
            }
            (KeyCode::Right | KeyCode::Char('l'), State::Focused(9..13)) => {
                self.change_selection_rel(-1);
            }
            (KeyCode::Left | KeyCode::Char('h'), State::Focused(1..5)) => {
                self.change_selection_rel(-1)
            }
            (KeyCode::Left | KeyCode::Char('h'), State::Focused(8..12)) => {
                self.change_selection_rel(1);
            }
            (KeyCode::Up | KeyCode::Char('k'), State::Focused(5..9)) => {
                self.change_selection_rel(-1)
            }
            (KeyCode::Up | KeyCode::Char('k'), State::Focused(12..16)) => {
                self.change_selection_rel(1)
            }
            (KeyCode::Down | KeyCode::Char('j'), State::Focused(4..8)) => {
                self.change_selection_rel(1)
            }
            (KeyCode::Down | KeyCode::Char('j'), State::Focused(13..16) | State::Focused(0)) => {
                self.change_selection_rel(-1)
            }
            (_, _) => {}
        }
    }

    pub fn change_selection(&mut self, new_selection_idx: usize) {
        if let State::Focused(old_idx) = self.selected {
            self.fields[old_idx].selected = false;
            self.selected = State::Focused(new_selection_idx);
            self.fields[new_selection_idx].selected = true;
        }
    }

    // only possible if GameField is selected (because keybinds are only available if selected)
    pub fn change_selection_rel(&mut self, new_selection_idx_rel: i32) {
        if let State::Focused(old_idx) = self.selected {
            let new_selection_idx = match old_idx as i32 + new_selection_idx_rel {
                _idx @ ..0 => 15,
                _idx @ 16.. => 0,
                idx => idx as usize,
            };

            self.fields[old_idx].selected = false;
            self.selected = State::Focused(new_selection_idx);
            self.fields[new_selection_idx].selected = true;
        }
    }

    fn move_camel(&mut self, camel: CamelColor, to_field: usize) -> Result<(), ()> {
        if to_field >= self.fields.len() {
            return Err(());
        }

        let (old_pos, camel_index) = self
            .fields
            .iter()
            .enumerate()
            .find_map(|(field_idx, field)| {
                field
                    .camels
                    .iter()
                    .position(|&c| c == camel)
                    .map(|camel_idx| (field_idx, camel_idx))
            })
            .ok_or(())?;

        if old_pos == to_field {
            return Err(());
        }

        let moving_camels = self.fields[old_pos].camels.split_off(camel_index);

        self.fields[to_field].camels.extend(moving_camels);

        Ok(())
    }
}

impl Widget for &GameField {
    //TODO: figure out minimal size and make seperate screen for when size is too small
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (x_offset, y_offset) = (5, 3);
        let outer_corner_game_field = Rect::new(
            area.x + x_offset,
            area.y + y_offset,
            area.width - 2 * x_offset,
            area.height - 2 * y_offset,
        );

        let main_layout = Layout::vertical([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(3, 5),
            Constraint::Ratio(1, 5),
        ]);
        let [top_row_area, middle_area, bottom_row_area] =
            main_layout.areas(outer_corner_game_field);

        let middle_layout = Layout::horizontal([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(3, 5),
            Constraint::Ratio(1, 5),
        ]);
        let [left_col_area, _center_area, right_col_area] = middle_layout.areas(middle_area);

        let row_layout = Layout::horizontal([Constraint::Ratio(1, 5); 5]);
        let col_layout = Layout::vertical([Constraint::Ratio(1, 3); 3]);

        let top_row_rects = row_layout.split(top_row_area);
        let right_col_rects = col_layout.split(right_col_area);
        let bottom_row_rects = row_layout.split(bottom_row_area);
        let left_col_rects = col_layout.split(left_col_area);

        let mut camel_field_areas = [Rect::default(); 16];

        //  0-4
        for (i, &rect) in top_row_rects.iter().enumerate() {
            camel_field_areas[i] = rect;
        }

        // 5-7
        for (i, &rect) in right_col_rects.iter().enumerate() {
            camel_field_areas[5 + i] = rect;
        }

        //  8-12 (reversed order)
        for (i, &rect) in bottom_row_rects.iter().rev().enumerate() {
            camel_field_areas[8 + i] = rect;
        }

        // 13-15 (reversed order)
        for (i, &rect) in left_col_rects.iter().rev().enumerate() {
            camel_field_areas[13 + i] = rect;
        }

        let border_color = if let State::Focused(_) = self.selected {
            Color::LightBlue
        } else {
            Color::White
        };

        let simple_instructions = "   <q> - quit <?> - help   ";

        if cfg!(debug_assertions) {
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_top(format!(
                    "GameField,area:{},{},{},{}",
                    area.x, area.y, area.width, area.height
                ))
                .title_bottom(Line::from(simple_instructions).centered())
                .render(area, buf);
        } else {
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_top("GameField")
                .title_bottom(Line::from(simple_instructions).centered())
                .render(area, buf);
        }

        for (i, field) in self.fields.iter().enumerate() {
            field.render(camel_field_areas[i], buf);
        }
    }
}
