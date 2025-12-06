use std::{array, io};

use palette::{IntoColor, Okhsv, Srgb};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Color,
    widgets::{Block, Widget},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum CamelColor {
    RED,
    GREEN,
    YELLOW,
    ORANGE,
    WHITE,
}

impl CamelColor {
    pub const fn rgb(&self) -> Color {
        match self {
            CamelColor::RED => Color::Rgb(255, 0, 0),
            CamelColor::GREEN => Color::Rgb(0, 255, 0),
            CamelColor::YELLOW => Color::Rgb(255, 255, 0),
            CamelColor::ORANGE => Color::Rgb(255, 165, 0),
            CamelColor::WHITE => Color::Rgb(255, 255, 255),
        }
    }
}

const CAMEL_HEIGHT: usize = 15;
const CAMEL_WIDTH: usize = 19;
#[rustfmt::skip]
const CAMEL_PATTERN: [[bool; CAMEL_WIDTH]; CAMEL_HEIGHT] = [
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

//TODO: render camel
/// camels are ordered from bottom to top
#[derive(Debug, Default)]
struct CamelField {
    camels: Vec<CamelColor>,
    index: usize,
    area: Rect,
}

impl CamelField {
    fn new(camels: Vec<CamelColor>, index: usize) -> Self {
        Self {
            camels,
            index,
            area: Rect::default(),
        }
    }

    //TODO:
    fn render_camels(&self, buf: &mut Buffer) {
        // to place the camels roughly in the middle of the field
        let (x_offset, y_offset) = (self.area.x + self.area.width / 4, self.area.y);
        for camel_idx in 0..self.camels.len() {
            let col = self.camels[camel_idx].rgb();
            let mut y = 0;

            let camel_stacking_offset = camel_idx * 2;

            while y < CAMEL_HEIGHT {
                // Process two vertical pixels at once except for odd remainder
                if y + 1 < CAMEL_HEIGHT {
                    for x in 0..CAMEL_WIDTH {
                        let char_x = x as u16 + x_offset;
                        // divide by 2 because we handle 2 pixels at once
                        let char_y = (y / 2) as u16 + y_offset + camel_stacking_offset as u16;

                        match (CAMEL_PATTERN[y][x], CAMEL_PATTERN[y + 1][x]) {
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
                    y += 2; // Increment by 2 since we processed 2 rows
                } else {
                    // Handle odd remainder row
                    for x in 0..CAMEL_WIDTH {
                        if CAMEL_PATTERN[y][x] {
                            let char_x = x as u16 + x_offset;
                            let char_y = (y / 2) as u16 + y_offset + camel_stacking_offset as u16;

                            buf[(char_x, char_y)].set_char('▀').set_fg(col);
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
        Block::bordered()
            .title_top(format!(
                "Field {}, area:{},{},{},{}",
                self.index, self.area.x, self.area.y, self.area.width, self.area.height
            ))
            .render(area, buf);
        self.render_camels(buf);
    }
}

#[derive(Debug)]
struct GameField {
    fields: [CamelField; 16],
    area: Rect,
}

impl GameField {
    fn new(game_field_area: Rect) -> Self {
        let (x_offset, y_offset) = (5, 3);
        let outer_corner_game_field = Rect::new(
            game_field_area.x + x_offset,
            game_field_area.y + y_offset,
            game_field_area.width - 2 * x_offset,
            game_field_area.height - 2 * y_offset,
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

        let mut fields: [CamelField; 16] = array::from_fn(|i| CamelField::new(Vec::new(), i));

        //  0-4
        for (i, &rect) in top_row_rects.iter().enumerate() {
            fields[i].area = rect;
        }

        // 5-7
        for (i, &rect) in right_col_rects.iter().enumerate() {
            fields[5 + i].area = rect;
        }

        //  8-12 (reversed order)
        for (i, &rect) in bottom_row_rects.iter().rev().enumerate() {
            fields[8 + i].area = rect;
        }

        // 13-15 (reversed order)
        for (i, &rect) in left_col_rects.iter().rev().enumerate() {
            fields[13 + i].area = rect;
        }

        Self {
            fields,
            area: game_field_area,
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
    fn render(self, _area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top(format!(
                "GameField,area:{},{},{},{}",
                self.area.x, self.area.y, self.area.width, self.area.height
            ))
            .render(self.area, buf);
        for field in &self.fields {
            field.render(field.area, buf);
        }
    }
}

#[derive(Debug)]
struct CamelStateField {
    area: Rect,
}

impl CamelStateField {
    fn new(camel_state_area: Rect) -> Self {
        CamelStateField {
            area: camel_state_area,
        }
    }
}

impl Widget for &CamelStateField {
    fn render(self, _area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered().title_top("Kamele").render(self.area, buf);
    }
}

#[derive(Debug)]
struct ProbabilitiesField {
    area: Rect,
}

impl ProbabilitiesField {
    fn new(probabilities_field_area: Rect) -> Self {
        ProbabilitiesField {
            area: probabilities_field_area,
        }
    }
}

impl Widget for &ProbabilitiesField {
    fn render(self, _area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top("Wahrscheinlichkeiten")
            .render(self.area, buf);
    }
}

//TODO: implement actual stuff
#[derive(Debug)]
struct NumbersField {
    probabilities: ProbabilitiesField,
    camel_state: CamelStateField,
    area: Rect,
}

impl Widget for &NumbersField {
    fn render(self, _area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top("Numbers")
            .render(self.area, buf);
        self.probabilities.render(self.probabilities.area, buf);
        self.camel_state.render(self.camel_state.area, buf);
    }
}

impl NumbersField {
    fn new(numbers_area: Rect) -> Self {
        let numbers_layout = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            // Constraint::Percentage(10),
        ]);

        let [
            probability_area,
            camel_state_area, //, calculate_buttons_area
        ] = numbers_layout.areas(numbers_area);
        Self {
            probabilities: ProbabilitiesField::new(probability_area),
            camel_state: CamelStateField::new(camel_state_area),
            area: numbers_area,
        }
    }
}

#[derive(Debug)]
struct App {
    game_field: GameField,
    numbers_field: NumbersField,
    exit: bool,
}

impl App {
    fn new(area: Rect) -> Self {
        let general_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);
        let [numbers_area, game_area] = general_layout.areas(area);

        Self {
            game_field: GameField::new(game_area),
            exit: false,
            numbers_field: NumbersField::new(numbers_area),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        #[allow(clippy::single_match)]
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

pub fn color_from_oklab(hue: f32, saturation: f32, value: f32) -> Color {
    let color: Srgb = Okhsv::new(hue, saturation, value).into_color();
    let color = color.into_format();
    Color::Rgb(color.red, color.green, color.blue)
}

impl Widget for &App {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        (&self.numbers_field).render(self.numbers_field.area, buf);
        (&self.game_field).render(self.game_field.area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let area = terminal.get_frame().area();
    let mut app = App::new(area);

    app.game_field.fields[0].camels.push(CamelColor::RED);
    app.game_field.fields[0].camels.push(CamelColor::GREEN);
    app.game_field.fields[3].camels.push(CamelColor::YELLOW);
    app.game_field.fields[10].camels.push(CamelColor::ORANGE);
    app.game_field.fields[0].camels.push(CamelColor::WHITE);

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
