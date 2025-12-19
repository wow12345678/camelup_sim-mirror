use std::{fs::File, io};

use self::{
    camelfield::CamelField,
    numbersfield::{CamelState, ProbabilitiesField},
};
use camelfield::CamelColor;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Clear, Widget},
};
use simplelog::{Config, LevelFilter, WriteLogger};

mod camelfield;
mod numbersfield;

#[derive(Debug)]
pub struct GameState {
    fields: [CamelField; 16],
    selected_color: usize,
    selected_field: usize,
    camel_round_info: [CamelState; 5],
    rolled_dice: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            fields: Default::default(),
            selected_color: 0,
            selected_field: 0,
            camel_round_info: CamelColor::all().map(CamelState::new),
            rolled_dice: 0,
        }
    }
}

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
    InvalidConfiguration,
}

// TODO: centralize game_state
impl GameState {
    pub fn init(config: &Vec<(u8, CamelColor)>) -> GameState {
        let mut init = GameState::default();

        init.fields[init.selected_field].selected = true;
        init.camel_round_info[init.selected_color].selected = true;

        for (i, col) in config {
            init.fields[*i as usize].camels.push(*col);
            init.camel_round_info[Into::<usize>::into(*col)].start_pos = *i;
        }

        init
    }

    pub fn move_camel(&mut self, camel: CamelColor, to_field: usize) -> Result<(), MoveError> {
        let mut camel_state = self.camel_round_info[Into::<usize>::into(camel)];
        if to_field >= self.fields.len() {
            return Err(MoveError::InvalidMove);
        }

        if camel_state.has_moved {
            return Err(MoveError::InvalidMove);
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
            .ok_or(MoveError::InvalidConfiguration)?;

        if old_pos > to_field || to_field - old_pos > 3 {
            return Err(MoveError::InvalidMove);
        }

        if old_pos == to_field {
            return Err(MoveError::InvalidMove);
        }

        camel_state.has_moved = true;

        let moving_camels = self.fields[old_pos].camels.split_off(camel_index);
        for cam in &moving_camels {
            self.camel_round_info[Into::<usize>::into(*cam)].pos_round_add =
                (to_field - old_pos) as i32;
        }

        self.fields[to_field].camels.extend(moving_camels);

        Ok(())
    }

    pub fn move_selected_field_rel(&mut self, by: i32) {
        let camel: CamelColor = self.selected_color.into();
        let old_idx = self.selected_field;
        let new_selection_idx = match old_idx as i32 + by {
            _idx @ ..0 => 15,
            _idx @ 16.. => 0,
            idx => idx as usize,
        };

        self.selected_field = new_selection_idx;
        self.fields[old_idx].selected = false;
        self.fields[new_selection_idx].selected = true;

        // has to be some
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
            .expect("This should always be Some, since there is always 1 camel of each color");

        let moving_camels = self.fields[old_pos].camels.iter().skip(camel_index);

        for cam in moving_camels {
            self.camel_round_info[Into::<usize>::into(*cam)].pos_round_add += by;
        }
    }

    pub fn move_selected_color(&mut self, new_color: usize) {
        self.camel_round_info[self.selected_color].selected = false;
        self.camel_round_info[new_color].selected = true;
        self.selected_color = new_color;
    }

    pub fn move_selected_color_rel(&mut self, by: i32) {
        let old_idx = self.selected_color;
        let new_selection_idx = match old_idx as i32 + by {
            _idx @ 5.. => 0,
            _idx @ ..0 => 4,
            idx => idx as usize,
        };
        self.camel_round_info[self.selected_color].selected = false;
        self.selected_color = new_selection_idx;
        self.camel_round_info[new_selection_idx].selected = true;
    }


    fn render_camel_info_field(&self, area: Rect, buf: &mut Buffer, selected_window: GeneralWindow)
    where
        Self: Sized,
    {
        let border_color = if let GeneralWindow::NumberField = selected_window {
            Color::LightBlue
        } else {
            Color::White
        };

        let outer_line = Block::bordered()
            .border_style(Style::default().fg(border_color))
            .title_top("Kamele");

        let inner_area = outer_line.inner(area);

        outer_line.render(area, buf);

        let constraints = [
            vec![Constraint::Length(1)],
            vec![Constraint::Length(4); 5],
            vec![Constraint::Length(1)],
        ]
        .concat();

        let camel_layout = Layout::vertical(constraints);

        let rows: [_; 7] = camel_layout.areas(inner_area);

        Line::from(format!("Round {}", 0))
            .centered()
            .render(rows[0], buf);

        for (i, r) in rows.iter().take(6).skip(1).enumerate() {
            self.camel_round_info[i].render(*r, buf);
        }

        Line::from(format!("Remaining Dice {dice}/5", dice = self.rolled_dice))
            .centered()
            .render(rows[6], buf);
    }

    fn render_game_field(&self, area: Rect, buf: &mut Buffer, selected_window: GeneralWindow)
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

        let border_color = if let GeneralWindow::GameField = selected_window {
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum GeneralWindow {
    NumberField,
    GameField,
    Help,
}

#[derive(Debug)]
struct App {
    probabilities: ProbabilitiesField,
    game_state: GameState,
    exit: bool,
    selected_window: GeneralWindow,
    window_stack: Vec<GeneralWindow>,
}

impl App {
    fn new(config: &Vec<(u8, CamelColor)>) -> Self {
        // initialize GameState here
        // TODO: setup period of game
        let init_game_state = GameState::init(config);

        Self {
            exit: false,
            game_state: init_game_state,
            probabilities: ProbabilitiesField::default(),
            selected_window: GeneralWindow::NumberField,
            window_stack: Vec::new(),
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

    fn render_help_popup(&self, area: Rect, buf: &mut Buffer) {
        let popup = Block::bordered().title(Line::from("   Help   ").centered());
        Clear.render(area, buf);
        popup.render(area, buf);
    }

    // TODO: think about error handling
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        const SELECTION_KEYBINDS: [char; 5] = ['b', 'g', 'y', 'o', 'w'];

        match (key_event.code, self.selected_window) {
            // quit
            (
                KeyCode::Char('q') | KeyCode::Esc,
                GeneralWindow::GameField | GeneralWindow::NumberField,
            ) => self.exit(),
            // help window
            (KeyCode::Char('?'), GeneralWindow::Help) => {
                if let Some(prev_window) = self.window_stack.pop() {
                    self.selected_window = prev_window;
                }
            }
            (KeyCode::Char('?'), old_window) => {
                self.window_stack.push(old_window);
                self.selected_window = GeneralWindow::Help;
            }
            // quit help window
            (KeyCode::Char('q') | KeyCode::Esc, GeneralWindow::Help) => {
                if let Some(prev_window) = self.window_stack.pop() {
                    self.selected_window = prev_window;
                }
            }
            // hotkeys
            (KeyCode::Char(c), _) if SELECTION_KEYBINDS.contains(&c) => {
                self.game_state
                    .move_selected_color(CamelColor::from_char_to_usize(c));
            }
            // switch between main windows
            (KeyCode::Tab, _) => {
                if self.selected_window == GeneralWindow::GameField {
                    self.focus_window(GeneralWindow::NumberField);
                } else {
                    self.focus_window(GeneralWindow::GameField);
                }
            }
            (key, GeneralWindow::GameField) => {
                // TODO: Error handling
                let _ = self.handle_game_field_keys(key);
            }
            (key, GeneralWindow::NumberField) => {
                self.handle_number_field_keys(key);
            }
            (_, _) => {}
        }
    }

    fn handle_number_field_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                self.game_state.move_selected_color_rel(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.game_state.move_selected_color_rel(-1);
            }
            KeyCode::Enter => {
                self.game_state.move_selected_color_rel(-1);
            }
            _ => {}
        }
    }

    fn handle_game_field_keys(&mut self, key: KeyCode) -> Result<(), MoveError> {
        match (key, self.game_state.selected_field) {
            (KeyCode::Enter, _) => self.game_state.move_camel(
                self.game_state.selected_color.into(),
                self.game_state.selected_field,
            ),
            (KeyCode::Right | KeyCode::Char('l'), 0..4) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Right | KeyCode::Char('l'), 9..13) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (KeyCode::Left | KeyCode::Char('h'), 1..5) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (KeyCode::Left | KeyCode::Char('h'), 8..12) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Up | KeyCode::Char('k'), 5..9) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (KeyCode::Up | KeyCode::Char('k'), 12..16) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Down | KeyCode::Char('j'), 4..8) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Down | KeyCode::Char('j'), 13..16 | 0) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (_, _) => Ok(()),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn focus_window(&mut self, win: GeneralWindow) {
        self.selected_window = win;
    }

    // TODO: make config Configuration type rather than Vec
    fn init(&mut self, config: Vec<(u8, CamelColor)>) {
        let init_game_state = GameState::init(&config);
    }
}

/// Create a centered rect using up certain percentage of the available rect
fn centered_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let general_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);
        let [numbers_area, game_area] = general_layout.areas(area);

        let numbers_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [probability_area, camel_state_area] = numbers_layout.areas(numbers_area);

        self.game_state
            .render_game_field(game_area, buf, self.selected_window);
        self.game_state
            .render_camel_info_field(camel_state_area, buf, self.selected_window);
        
        self.probabilities.render(probability_area, buf);

        if let GeneralWindow::Help = self.selected_window {
            let center_area = centered_area(area, 60, 40);
            self.render_help_popup(center_area, buf);
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let init_config = vec![
        (0, CamelColor::Blue),
        (0, CamelColor::Green),
        (1, CamelColor::White),
        (1, CamelColor::Yellow),
        (2, CamelColor::Orange),
    ];
    let mut app = App::new(&init_config);
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )
    .unwrap();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
