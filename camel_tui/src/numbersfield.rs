use ratatui::{
    buffer::Buffer, crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, style::{Color, Style}, widgets::{Block, Widget}
};

#[derive(Debug)]
pub struct CamelStateField {
    selected: bool,
}

impl CamelStateField {
    fn new() -> Self {
        Self { selected: true }
    }
}

impl Widget for &CamelStateField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border_color = if self.selected {
            Color::LightBlue
        }else {
            Color::White
        };

        Block::bordered()
            .border_style(Style::default().fg(border_color))
            .title_top("Kamele").render(area, buf);
    }
}

#[derive(Debug)]
struct ProbabilitiesField {}

impl ProbabilitiesField {
    fn new() -> Self {
        ProbabilitiesField {}
    }
}

impl Widget for &ProbabilitiesField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top("Wahrscheinlichkeiten")
            .render(area, buf);
    }
}

//TODO: implement actual stuff
#[derive(Debug)]
pub struct NumbersField {
    probabilities: ProbabilitiesField,
    camel_state: CamelStateField,
}

impl NumbersField {
    pub fn focus(&mut self) {
        if !self.camel_state.selected {
            self.camel_state.selected = !self.camel_state.selected;
        } else {
            panic!("The GameField should be unfocused if the method is called")
        }
    }

    pub fn unfocus(&mut self) {
        if self.camel_state.selected {
            self.camel_state.selected = !self.camel_state.selected;
        } else {
            panic!("The GameField should be focused if the method is called")
        }
    }

    pub fn handle_key_event(&self, key: KeyCode) {
        todo!()
    }

    pub fn new() -> Self {
        Self {
            probabilities: ProbabilitiesField::new(),
            camel_state: CamelStateField::new(),
        }
    }

}

impl Widget for &NumbersField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let numbers_layout = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);

        let [
            probability_area,
            camel_state_area,
        ] = numbers_layout.areas(area);

        Block::bordered().title_top("Numbers").render(area, buf);

        self.probabilities.render(probability_area, buf);
        self.camel_state.render(camel_state_area, buf);
    }
}
