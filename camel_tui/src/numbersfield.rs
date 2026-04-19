use std::{cmp::Ordering, fmt::Display, sync::mpsc::Sender, thread};

use calc::EffectCardType;
use throbber_widgets_tui::{BRAILLE_SIX_DOUBLE, Throbber, ThrobberState, WhichUse};

use crate::{camelfield::CamelColor, gamestate::GameState};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, ToSpan},
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

#[derive(Debug, Clone)]
pub struct EffectCardState {
    pub effect_type: EffectCardType,
    pub placements: Vec<u8>,
    pub selected: bool,
}

impl EffectCardState {
    pub fn new(effect_type: EffectCardType) -> Self {
        Self {
            effect_type,
            placements: Vec::new(),
            selected: false,
        }
    }

    pub fn toggle_placement(&mut self, field: u8) {
        if let Some(idx) = self.placements.iter().position(|&p| p == field) {
            self.placements.remove(idx);
        } else {
            self.placements.push(field);
        }
    }

    pub fn has_placement(&self, field: u8) -> bool {
        self.placements.contains(&field)
    }

    pub fn clear_placements(&mut self) {
        self.placements.clear();
    }
}

impl Display for EffectCardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.effect_type {
            EffectCardType::Oasis => "Oasis",
            EffectCardType::Desert => "Desert",
        };
        if self.placements.is_empty() {
            write!(f, "{}: -", name)
        } else {
            let positions: Vec<String> = self.placements.iter().map(|p| p.to_string()).collect();
            write!(f, "{}: {}", name, positions.join(", "))
        }
    }
}

impl Widget for &EffectCardState {
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

        let text_color = match self.effect_type {
            EffectCardType::Oasis => Color::Green,
            EffectCardType::Desert => Color::Red,
        };

        Line::from(format!("{self}"))
            .style(Style::default().fg(text_color))
            .render(inner_area, buf);
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
            .title_top("Camels");

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

    pub game_win_probabilities: Option<[[f32; 5]; 5]>,
    pub game_win_calculating: bool,
    pub game_win_sender: Sender<[[f32; 5]; 5]>,
    pub(crate) game_win_calc_thread: Option<thread::JoinHandle<()>>,

    pub round_throbber_state: ThrobberState,
    pub game_win_throbber_state: ThrobberState,
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
            .field("game_win_probabilities", &self.game_win_probabilities)
            .field("game_win_calculating", &self.game_win_calculating)
            .field("game_win_sender", &self.game_win_sender)
            .field(
                "game_win_calc_thread",
                &self.game_win_calc_thread.as_ref().map(|_| "<JoinHandle>"),
            )
            .field("round_throbber_state", &self.round_throbber_state)
            .field("game_win_throbber_state", &self.game_win_throbber_state)
            .finish()
    }
}

impl ProbabilitiesField {
    pub fn start_probability_calculations(&mut self, game_state: &GameState) {
        if let Some(handle) = self.calc_thread.take() {
            let _ = handle.join();
        }

        let configuration = GameState::convert_game_state_configuration(game_state);
        // log::debug!("{configuration:?}");
        let tx = self.sender.clone();

        let handle = thread::Builder::new()
            .name("probability-calc".to_string())
            .spawn(move || {
                let res = calc::simulate_round(configuration);
                let leaderboard = res.weighted_leaderboard();
                let total: u128 = leaderboard[0].iter().sum();
                let res = leaderboard.map(|row| row.map(|elem| elem as f32 / total as f32));

                let _ = tx.send(res);
            })
            .expect("Failed to spawn probability calculation thread");

        self.calc_thread = Some(handle);
        self.calculating = true;
    }

    pub fn update_probabilities(&mut self, new_probabilities: [[f32; 5]; 5]) {
        self.probabilities = Some(new_probabilities);
    }

    pub fn start_game_win_calculations(&mut self, game_state: &GameState) {
        if let Some(handle) = self.game_win_calc_thread.take() {
            let _ = handle.join();
        }

        let configuration = GameState::convert_game_state_configuration(game_state);
        let tx = self.game_win_sender.clone();

        let handle = thread::Builder::new()
            .name("game-win-calc".to_string())
            .spawn(move || {
                let res = calc::simulate_rounds(configuration);
                let weighted = res.weighted_leaderboard();
                let total: u128 = weighted[0].iter().sum();
                let res = weighted.map(|row| row.map(|elem| elem as f32 / total as f32));

                let _ = tx.send(res);
            })
            .expect("Failed to spawn game win calculation thread");

        self.game_win_calc_thread = Some(handle);
        self.game_win_calculating = true;
    }

    pub fn update_game_win_probabilities(&mut self, new_probabilities: [[f32; 5]; 5]) {
        self.game_win_probabilities = Some(new_probabilities);
    }

    /// Takes ownership of the calculation thread handle for cleanup.
    /// Used when the app is shutting down to ensure threads are properly joined.
    pub fn take_thread(&mut self) -> Option<thread::JoinHandle<()>> {
        self.calc_thread.take()
    }

    pub fn take_game_win_thread(&mut self) -> Option<thread::JoinHandle<()>> {
        self.game_win_calc_thread.take()
    }

    pub fn tick_throbbers(&mut self) {
        if self.calculating {
            self.round_throbber_state.calc_next();
        }
        if self.game_win_calculating {
            self.game_win_throbber_state.calc_next();
        }
    }
}

impl Widget for &ProbabilitiesField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
        let [round_area, game_win_area] = layout.areas(area);

        ProbabilitiesField::render_probability_table(
            "Round Probabilities",
            self.probabilities,
            self.calculating,
            &self.round_throbber_state,
            round_area,
            buf,
        );
        ProbabilitiesField::render_probability_table(
            "Game Win Probabilities",
            self.game_win_probabilities,
            self.game_win_calculating,
            &self.game_win_throbber_state,
            game_win_area,
            buf,
        );
    }
}

impl ProbabilitiesField {
    fn render_probability_table(
        title: &str,
        probabilities: Option<[[f32; 5]; 5]>,
        calculating: bool,
        throbber_state: &ThrobberState,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let display_title = if calculating {
            let throbber = Throbber::default()
                .throbber_set(BRAILLE_SIX_DOUBLE)
                .use_type(WhichUse::Spin);
            let symbol = throbber.to_symbol_span(throbber_state);
            Line::from(vec![
                " ".into(),
                title.into(),
                " ".into(),
                symbol,
                " ".into(),
            ])
        } else {
            Line::from(vec![" ".into(), title.to_span(), "    ".into()])
        };

        let outer_line = Block::bordered().title_top(display_title);
        let inner_area = outer_line.inner(area);
        outer_line.render(area, buf);

        let header = Row::new(
            [" ".to_string()]
                .into_iter()
                .chain(CamelColor::all().map(|c| c.to_string())),
        );
        let col_layout = [vec![Constraint::Max(2)], vec![Constraint::Min(5); 5]].concat();

        let rows: Vec<Row<'_>>;

        if let Some(probs) = probabilities {
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
        Table::new(rows, col_layout)
            .header(header)
            .render(inner_area, buf);
    }
}
