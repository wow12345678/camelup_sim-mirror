use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Widget},
};

#[derive(Debug)]
struct CamelStateField {
}

impl CamelStateField {
    fn new() -> Self {
        CamelStateField {
        }
    }
}

impl Widget for &CamelStateField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered().title_top("Kamele").render(area, buf);
    }
}

#[derive(Debug)]
struct ProbabilitiesField {
}

impl ProbabilitiesField {
    fn new() -> Self {
        ProbabilitiesField {
        }
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

impl Widget for &NumbersField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let numbers_layout = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            // Constraint::Percentage(10),
        ]);

        let [
            probability_area,
            camel_state_area, //, calculate_buttons_area
        ] = numbers_layout.areas(area);

        Block::bordered()
            .title_top("Numbers")
            .render(area, buf);

        self.probabilities.render(probability_area, buf);
        self.camel_state.render(camel_state_area, buf);
    }
}

impl NumbersField {
    pub fn new() -> Self {
        Self {
            probabilities: ProbabilitiesField::new(),
            camel_state: CamelStateField::new(),
        }
    }
}
