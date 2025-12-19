use std::{
    fmt::Display,
    thread::{self, JoinHandle},
};

use crate::{camelfield::CamelColor, gamestate::GameState};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Row, Table, Widget},
};

#[derive(Debug, Clone, Copy)]
pub struct CamelState {
    pub camel_color: CamelColor,
    pub start_pos: u8,
    pub pos_round_add: i32,
    pub selected: bool,
    pub has_moved: bool,
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
            "{color}: {start_pos} -> {end_pos} | {sign}{increment}",
            color = self.camel_color,
            start_pos = self.start_pos,
            end_pos = self.start_pos as i32 + self.pos_round_add,
            sign = if self.pos_round_add >= 0 { '+' } else { ' ' },
            increment = self.pos_round_add
        )
    }
}

impl CamelState {
    pub fn new(camel_color: CamelColor) -> Self {
        Self {
            camel_color,
            start_pos: 0,
            pos_round_add: 0,
            selected: false,
            has_moved: false,
        }
    }
}

#[derive(Debug)]
pub enum State {
    Focused(usize),
    Unfocused(usize),
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
pub struct ProbabilitiesField {
    probabilities: [[f32; 5]; 5],
    calculated: bool,
    handle: Option<JoinHandle<[[u32; 5]; 5]>>,
}

impl ProbabilitiesField {
    pub fn calculate_probabilties(&mut self, game_state: &GameState) {
        let configuration = GameState::convert_game_state_configuration(game_state);

        let handle = thread::spawn(move || {
            let res = calc::simulate_rounds(configuration);
            calc::aggragate_placements(res.placements())
        });

        self.handle = Some(handle);
    }

    pub fn update_probabilities(&mut self, probabilities: [[f32; 5]; 5]) {
        todo!()
    }
}

impl Default for ProbabilitiesField {
    fn default() -> Self {
        ProbabilitiesField {
            probabilities: [[0.0; 5]; 5],
            calculated: false,
            handle: None,
        }
    }
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
