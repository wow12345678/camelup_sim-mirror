use crate::gamestate::MoveError;
use std::{fs::File, io, time::Duration};

use self::{
    camelfield::CamelField,
    gamestate::GameState,
    numbersfield::{CamelState, ProbabilitiesField},
};
use camelfield::CamelColor;

use log::debug;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{Block, Clear, Widget},
};
use simplelog::{Config, LevelFilter, WriteLogger};

mod camelfield;
mod gamestate;
mod numbersfield;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GeneralWindow {
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

        let mut app = App {
            exit: false,
            game_state: init_game_state,
            probabilities: ProbabilitiesField::default(),
            selected_window: GeneralWindow::NumberField,
            window_stack: Vec::new(),
        };

        app.probabilities.calculate_probabilties(&app.game_state);
        app
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            if self
                .probabilities
                .handle
                .as_ref()
                .is_some_and(|j| j.is_finished())
            {
                let handle = self.probabilities.handle.take().unwrap();
                let res = handle.join().unwrap();
                self.probabilities.update_probabilities(res);
                self.probabilities.calculating = false;
            }

            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
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
            _ => {}
        }
    }

    fn handle_game_field_keys(&mut self, key: KeyCode) -> Result<(), MoveError> {
        match (key, self.game_state.selected_field) {
            (KeyCode::Enter, _) => {
                let res = self.game_state.move_camel(
                    self.game_state.selected_color.into(),
                    self.game_state.selected_field,
                );
                if res.is_ok() {
                    self.probabilities.calculate_probabilties(&self.game_state);
                    self.game_state.add_dice_rolled();
                }
                res
            }
            (KeyCode::Right | KeyCode::Char('l'), 0..2 | 14..16) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Right | KeyCode::Char('l'), 7..11) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (KeyCode::Left | KeyCode::Char('h'), 0..3 | 15) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (KeyCode::Left | KeyCode::Char('h'), 6..10) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Up | KeyCode::Char('k'), 3..7) => {
                self.game_state.move_selected_field_rel(-1);
                Ok(())
            }
            (KeyCode::Up | KeyCode::Char('k'), 10..14) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Down | KeyCode::Char('j'), 2..6) => {
                self.game_state.move_selected_field_rel(1);
                Ok(())
            }
            (KeyCode::Down | KeyCode::Char('j'), 11..15) => {
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
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )
    .unwrap();

    let mut terminal = ratatui::init();
    let init_config = vec![
        (3, CamelColor::Blue),
        (3, CamelColor::White),
        (3, CamelColor::Orange),
        (3, CamelColor::Green),
        (3, CamelColor::Yellow),
    ];
    let mut app = App::new(&init_config);


    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
