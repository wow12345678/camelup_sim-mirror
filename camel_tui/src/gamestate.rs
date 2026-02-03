use crate::{CamelColor, CamelField, CamelState, GeneralWindow};
use MoveError::{InvalidConfiguration, InvalidMove};

use calc::Configuration;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Widget},
};

#[derive(Debug)]
pub struct GameState {
    fields: [CamelField; 16],
    pub selected_color: usize,
    pub selected_field: usize,
    camel_round_info: [CamelState; 5],
    rolled_dice: usize,
    round_number: u8,
    pub game_period: GamePeriod,
}

impl Default for GameState {
    fn default() -> Self {
        let mut fields: [CamelField; 16] = Default::default();

        for i in 0..16 {
            fields[i].index = (i + 2) % 16;
            fields[i].board_index = i;
        }

        // first field in the game
        fields[0].selected = true;

        //default camel selected
        let mut camel_round_info = CamelColor::all().map(CamelState::new);
        camel_round_info[0].selected = true;
        
        Self {
            fields,
            selected_color: 0,
            selected_field: 0,
            camel_round_info,
            rolled_dice: 0,
            game_period: GamePeriod::Setup,
            round_number: 0,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum GamePeriod {
    #[default]
    Setup,
    Game,
}

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
    InvalidConfiguration,
}

#[derive(Debug)]
pub enum PlaceError {
    InvalidIndex,
    InvalidColor,
}

#[derive(Debug)]
pub enum PlayerActionError {
    MoveError(MoveError),
    PlaceError(PlaceError),
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::InvalidMove => write!(f, "invalid move"),
            MoveError::InvalidConfiguration => write!(f, "invalid configuration"),
        }
    }
}

impl std::fmt::Display for PlaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceError::InvalidIndex => write!(f, "invalid index"),
            PlaceError::InvalidColor => write!(f, "invalid color"),
        }
    }
}

impl std::fmt::Display for PlayerActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerActionError::MoveError(e) => write!(f, "move error: {}", e),
            PlayerActionError::PlaceError(e) => write!(f, "place error: {}", e),
        }
    }
}

impl std::error::Error for MoveError {}
impl std::error::Error for PlaceError {}
impl std::error::Error for PlayerActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PlayerActionError::MoveError(e) => Some(e),
            PlayerActionError::PlaceError(e) => Some(e),
        }
    }
}

impl From<MoveError> for PlayerActionError {
    fn from(err: MoveError) -> Self {
        PlayerActionError::MoveError(err)
    }
}

impl From<PlaceError> for PlayerActionError {
    fn from(err: PlaceError) -> Self {
        PlayerActionError::PlaceError(err)
    }
}

impl GameState {
    pub fn round_finished(&self) -> bool {
        self.rolled_dice == 5
    }

    pub fn add_camel(&mut self, selected_color: usize, selected_field: usize) {
        self.fields[selected_field]
            .camels
            .push(selected_color.into());
    }

    pub fn convert_game_state_configuration(game_state: &GameState) -> Configuration {
        let positions =
            game_state
                .fields
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (idx, field)| {
                    let vec: Vec<(u8, calc::Color)> = field
                        .camels
                        .iter()
                        .map(|color| (idx as u8, Into::<calc::Color>::into(*color as usize)))
                        .collect();
                    acc.extend(vec);
                    acc
                });

        let colors: Vec<calc::Color> = game_state
            .camel_round_info
            .iter()
            .filter_map(|cam| {
                if !cam.has_moved {
                    Some(Into::<usize>::into(cam.camel_color).into())
                } else {
                    None
                }
            })
            .collect();

        Configuration::builder()
            .with_map(positions)
            .with_available_colors(colors)
            .build()
    }

    pub fn add_dice_rolled(&mut self) {
        self.rolled_dice += 1;
    }

    pub fn init(config: &Vec<(u8, CamelColor)>) -> GameState {
        let mut init = GameState::default();

        init.fields[init.selected_field].selected = true;
        init.camel_round_info[init.selected_color].selected = true;

        for (i, col) in config {
            init.fields[*i as usize].camels.push(*col);
            init.camel_round_info[Into::<usize>::into(*col)].start_pos = *i as u32;
        }

        init
    }

    pub fn move_camel(
        &mut self,
        camel: CamelColor,
        to_field: usize,
    ) -> Result<(), PlayerActionError> {
        let camel_state = self.camel_round_info[Into::<usize>::into(camel)];
        if to_field >= self.fields.len() {
            return Err(InvalidMove.into());
        }

        if camel_state.has_moved {
            return Err(InvalidMove.into());
        }

        let (old_pos, camel_index) = self.find_camel(camel).ok_or(InvalidConfiguration)?;

        if old_pos > to_field || to_field - old_pos > 3 {
            return Err(InvalidMove.into());
        }

        if old_pos == to_field {
            return Err(InvalidMove.into());
        }

        let camel_state = &mut self.camel_round_info[Into::<usize>::into(camel)];
        camel_state.has_moved = true;

        let moving_camels = self.fields[old_pos].camels.split_off(camel_index);
        for cam in &moving_camels {
            self.camel_round_info[Into::<usize>::into(*cam)].end_pos = to_field as u32;
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

        if let Some((old_pos, camel_index)) = self.find_camel(camel) {
            let moving_camels = self.fields[old_pos].camels.iter().skip(camel_index);

            for cam in moving_camels {
                self.camel_round_info[Into::<usize>::into(*cam)].end_pos = new_selection_idx as u32;
            }
        }
    }

    pub fn move_selected_color(&mut self, new_color: usize) {
        let old_color = self.selected_color;
        self.camel_round_info[old_color].selected = false;

        for cam_info in &mut self.camel_round_info {
            // TODO: maybe make this better
            if !cam_info.has_moved {
                cam_info.end_pos = cam_info.start_pos;
            }
        }

        self.camel_round_info[new_color].selected = true;
        self.selected_color = new_color;
    }

    fn find_camel(&self, camel: CamelColor) -> Option<(usize, usize)> {
        self.fields
            .iter()
            .enumerate()
            .find_map(|(field_idx, field)| {
                field
                    .camels
                    .iter()
                    .position(|&c| c == camel)
                    .map(|camel_idx| (field_idx, camel_idx))
            })
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

    pub fn new_round(&mut self) {
        self.round_number += 1;
        for cam_info in &mut self.camel_round_info {
            cam_info.start_pos = cam_info.end_pos;
            cam_info.has_moved = false;
        }
        self.rolled_dice = 0;
    }

    pub fn render_camel_info_field(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selected_window: GeneralWindow,
    ) where
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

        let round_string = if self.round_number == 0 {
            "Initialization Round".to_string()
        } else {
            format!("Round {}", self.round_number)
        };

        Line::from(round_string).centered().render(rows[0], buf);

        for (i, r) in rows.iter().take(6).skip(1).enumerate() {
            self.camel_round_info[i].render(*r, buf);
        }

        Line::from(format!("Remaining Dice {dice}/5", dice = self.rolled_dice))
            .centered()
            .render(rows[6], buf);
    }

    pub fn render_game_field(&self, area: Rect, buf: &mut Buffer, selected_window: GeneralWindow)
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

        let simple_instructions = "   <q> - quit  <Tab> - switch  <?> - help   ";

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
            //wierd indexing because real game starts at second field
            field.render(camel_field_areas[(i + 2) % 16], buf);
        }
    }

    pub(crate) fn camel_info(&mut self, camel_color: usize) -> Option<&mut CamelState> {
        self.camel_round_info
            .iter_mut()
            .filter_map(|info| {
                if Into::<usize>::into(info.camel_color) == camel_color {
                    Some(info)
                } else {
                    None
                }
            })
            .next()
    }
}
