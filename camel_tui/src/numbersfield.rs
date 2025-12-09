use std::fmt::Display;

use crate::gamefield::{CamelColor, State};

use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Row, Table, Widget},
};

#[derive(Debug, Clone, Copy)]
struct CamelState {
    camel_color: CamelColor,
    start_pos: u8,
    pos_round_add: u8,
    selected: bool,
}

impl Widget for CamelState {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border_color = if self.selected {
            Color::LightBlue
        } else {
            Color::White
        };

        let borders = Block::bordered().style(Style::default().fg(border_color));
        let inner_area = borders.inner(area);
        borders.render(area, buf);
        Line::from(format!("{self}"))
            // .style(
            //     Style::default()
            //         .fg(self.camel_color.text_color())
            //         .bg(self.camel_color.to_color()),
            // )
            .render(inner_area, buf);
    }
}

impl Display for CamelState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{color}: {start_pos} -> {end_pos} | +{increment}",
            color = self.camel_color,
            start_pos = self.start_pos,
            end_pos = self.start_pos + self.pos_round_add,
            increment = self.pos_round_add
        )
    }
}

impl CamelState {
    fn new(camel_color: CamelColor) -> Self {
        Self {
            camel_color,
            start_pos: 0,
            pos_round_add: 0,
            selected: false,
        }
    }
}

#[derive(Debug)]
pub struct CamelStateField {
    selected: State,
    rolled_dice: u8,
    camels: [CamelState; 5],
}

impl CamelStateField {
    fn new() -> Self {
        let mut camels: [CamelState; 5] = CamelColor::all().map(CamelState::new);

        camels[0].selected = true;

        Self {
            selected: State::Focused(0),
            rolled_dice: 0,
            camels,
        }
    }
}

impl Widget for &CamelStateField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border_color = if let State::Focused(_) = self.selected {
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
            self.camels[i].render(*r, buf);
        }

        Line::from(format!("Remaining Dice {dice}/5", dice = self.rolled_dice))
            .centered()
            .render(rows[6], buf);
    }
}

#[derive(Debug)]
struct ProbabilitiesField {
    probabilities: [[f32; 5]; 5],
    calculated: bool,
}

impl ProbabilitiesField {
    fn new() -> Self {
        ProbabilitiesField {
            probabilities: [[0.0; 5]; 5],
            calculated: false,
        }
    }

    //TODO: integrate with calculator (2nd thread?) 
    //calculate with space
    fn calculate_probabilties(&mut self) {}
}

impl Widget for &ProbabilitiesField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let outer_line = Block::bordered().title_top("Wahrscheinlichkeiten");
        let inner_area = outer_line.inner(area);
        outer_line.render(area, buf);

        let header = Row::new(
            [" ".to_string()]
                .into_iter()
                .chain(CamelColor::all().map(|c| c.to_string())),
        );
        let layout = [Constraint::Min(5); 6];

        let empty_rows: Vec<Row<'_>> = (0..5)
            .map(|i| {
                Row::new(vec![
                    format!("{i}"),
                    "-".to_string(),
                    "-".to_string(),
                    "-".to_string(),
                    "-".to_string(),
                    "-".to_string(),
                ])
            })
            .collect();

        // let rows = self.probabilities.iter().map(|r| )
        Table::new(empty_rows, layout)
            .header(header)
            .render(inner_area, buf);
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
        if let State::Unfocused(idx) = self.camel_state.selected {
            self.camel_state.selected = State::Focused(idx);
        } else {
            panic!("The GameField should be unfocused if the method is called")
        }
    }

    pub fn unfocus(&mut self) {
        if let State::Focused(idx) = self.camel_state.selected {
            self.camel_state.selected = State::Unfocused(idx);
        } else {
            panic!("The GameField should be focused if the method is called")
        }
    }

    fn change_selection_rel(&mut self, new_selection_idx_rel: i32) {
        if let State::Focused(old_idx) = self.camel_state.selected {
            let new_selection_idx = match old_idx as i32 + new_selection_idx_rel {
                _idx @ 5.. => 0,
                _idx @ ..0 => 4,
                idx => idx as usize,
            };
            self.camel_state.camels[old_idx].selected = false;
            self.camel_state.selected = State::Focused(new_selection_idx);
            self.camel_state.camels[new_selection_idx].selected = true;
        }
    }

    pub fn change_selection(&mut self, new_selection_idx: usize) {
        if let State::Focused(old_idx) = self.camel_state.selected {
            self.camel_state.camels[old_idx].selected = false;
            self.camel_state.selected = State::Focused(new_selection_idx);
            self.camel_state.camels[new_selection_idx].selected = true;
        } else if let State::Unfocused(old_idx) = self.camel_state.selected {
            self.camel_state.camels[old_idx].selected = false;
            self.camel_state.selected = State::Unfocused(new_selection_idx);
            self.camel_state.camels[new_selection_idx].selected = true;
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                self.change_selection_rel(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.change_selection_rel(-1);
            }
            KeyCode::Enter => {
                self.change_selection_rel(-1);
            }
            _ => {}
        }
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
        let numbers_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [probability_area, camel_state_area] = numbers_layout.areas(area);

        Block::bordered().title_top("Numbers").render(area, buf);

        self.probabilities.render(probability_area, buf);
        self.camel_state.render(camel_state_area, buf);
    }
}
