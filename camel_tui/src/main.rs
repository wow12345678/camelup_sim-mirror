use crate::gamefield::GameField;
use crate::numbersfield::NumbersField;
use std::io;

use gamefield::CamelColor;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{Block, Clear, Widget},
};

mod gamefield;
mod numbersfield;

#[derive(Debug, PartialEq, Clone, Copy)]
enum GeneralWindow {
    KamelWindow,
    GameFieldWindow,
}

#[derive(Debug)]
struct App {
    game_field: GameField,
    numbers_field: NumbersField,
    exit: bool,
    show_help_popup: bool,
    selected_window: GeneralWindow,
}

impl App {
    fn new() -> Self {
        Self {
            game_field: GameField::new(),
            exit: false,
            numbers_field: NumbersField::new(),
            show_help_popup: false,
            selected_window: GeneralWindow::KamelWindow,
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, self.selected_window) {
            (KeyCode::Char('q'), _) => self.exit(),
            (KeyCode::Char('?'), _) => {
                self.show_help_popup = !self.show_help_popup;
            }
            (KeyCode::Tab, _) => {
                if self.selected_window == GeneralWindow::GameFieldWindow {
                    self.selected_window = GeneralWindow::KamelWindow;
                    self.game_field.unfocus();
                    self.numbers_field.focus();
                } else {
                    self.selected_window = GeneralWindow::GameFieldWindow;
                    self.numbers_field.unfocus();
                    self.game_field.focus();
                }
            }
            (key, GeneralWindow::KamelWindow) => {
                self.numbers_field.handle_key_event(key);
            }
            (key, GeneralWindow::GameFieldWindow) => {
                self.game_field.handle_key_event(key);
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
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

        self.numbers_field.render(numbers_area, buf);
        self.game_field.render(game_area, buf);

        if self.show_help_popup {
            let center_area = centered_area(area, 60, 40);
            self.render_help_popup(center_area, buf);
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.game_field.fields[0].camels.push(CamelColor::Red);
    app.game_field.fields[0].camels.push(CamelColor::Green);
    app.game_field.fields[0].camels.push(CamelColor::Yellow);
    app.game_field.fields[0].camels.push(CamelColor::Orange);
    app.game_field.fields[0].camels.push(CamelColor::White);
    app.game_field.fields[8].camels.push(CamelColor::Yellow);
    app.game_field.fields[10].camels.push(CamelColor::Yellow);

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
