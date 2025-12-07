use crate::numbersfield::NumbersField;
use crate::gamefield::GameField;
use std::io;

use gamefield::CamelColor;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

mod gamefield;
mod numbersfield;



#[derive(Debug)]
struct App {
    game_field: GameField,
    numbers_field: NumbersField,
    exit: bool,
}

impl App {
    fn new() -> Self {

        Self {
            game_field: GameField::new(),
            exit: false,
            numbers_field: NumbersField::new(),
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


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let general_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);
        let [numbers_area, game_area] = general_layout.areas(area);

        (&self.numbers_field).render(numbers_area, buf);
        (&self.game_field).render(game_area, buf);
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
