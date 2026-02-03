use std::{cmp::Ordering, fmt::Display, sync::mpsc::Sender, thread};

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
    pub start_pos: u32,
    pub end_pos: u32,
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
            .style(
                Style::default().fg(self.camel_color.to_color()),
                //         .bg(self.camel_color.to_color()),
            )
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
            end_pos = self.end_pos,
            sign = match self.start_pos.cmp(&self.end_pos) {
                Ordering::Less => {
                    '+'
                }
                Ordering::Equal => {
                    ' '
                }
                Ordering::Greater => {
                    '-'
                }
            },
            increment = self.end_pos.abs_diff(self.start_pos)
        )
    }
}

impl CamelState {
    pub fn new(camel_color: CamelColor) -> Self {
        Self {
            camel_color,
            start_pos: 0,
            end_pos: 0,
            selected: false,
            has_moved: false,
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
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

        Line::from(format!("Rolled Dice {dice}/5", dice = self.rolled_dice))
            .centered()
            .render(rows[6], buf);
    }
}

pub struct ProbabilitiesField {
    pub probabilities: Option<[[f32; 5]; 5]>,
    pub calculating: bool,
    pub sender: Sender<[[f32; 5]; 5]>,
    pub(crate) calc_thread: Option<thread::JoinHandle<()>>,
}

impl std::fmt::Debug for ProbabilitiesField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProbabilitiesField")
            .field("probabilities", &self.probabilities)
            .field("calculating", &self.calculating)
            .field("sender", &self.sender)
            .field(
                "calc_thread",
                &self.calc_thread.as_ref().map(|_| "<JoinHandle>"),
            )
            .finish()
    }
}

impl ProbabilitiesField {
    pub fn start_probability_calculations(&mut self, game_state: &GameState) {
        if let Some(handle) = self.calc_thread.take() {
            let _ = handle.join();
        }

        let configuration = GameState::convert_game_state_configuration(game_state);
        let tx = self.sender.clone();

        let handle = thread::Builder::new()
            .name("probability-calc".to_string())
            .spawn(move || {
                let res = calc::simulate_rounds(configuration);
                let game_states_count_all = res.placements().len();
                let res = res
                    .aggragated_leaderboard()
                    .map(|row| row.map(|elem| elem as f32 / game_states_count_all as f32));

                let _ = tx.send(res);
            })
            .expect("Failed to spawn probability calculation thread");

        self.calc_thread = Some(handle);
        self.calculating = true;
    }

    pub fn update_probabilities(&mut self, new_probabilities: [[f32; 5]; 5]) {
        self.probabilities = Some(new_probabilities);
    }

    /// Takes ownership of the calculation thread handle for cleanup.
    /// Used when the app is shutting down to ensure threads are properly joined.
    pub fn take_thread(&mut self) -> Option<thread::JoinHandle<()>> {
        self.calc_thread.take()
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

        let rows: Vec<Row<'_>>;

        if let Some(probs) = self.probabilities {
            rows = (0..5)
                .map(|i| {
                    Row::new(
                        [
                            vec![format!("{}.", i + 1)],
                            probs.iter().map(|r| r[i].to_string()).collect(),
                        ]
                        .concat(),
                    )
                })
                .collect();
        } else {
            rows = (0..5)
                .map(|i| {
                    Row::new(vec![
                        format!("{}.", i + 1),
                        "-".to_string(),
                        "-".to_string(),
                        "-".to_string(),
                        "-".to_string(),
                        "-".to_string(),
                    ])
                })
                .collect();
        }
        Table::new(rows, layout)
            .header(header)
            .render(inner_area, buf);
    }
}
