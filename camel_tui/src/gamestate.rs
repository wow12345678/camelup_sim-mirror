use crate::SelectionType;
use crate::asset_vec;
use crate::camelfield::{ARROW_LEFT, CamelFieldContent};
use crate::gameasset::GameAssetManager;
use crate::numbersfield::EffectCardState;
use crate::playererrors::{
    MoveError::{InvalidConfiguration, InvalidMove},
    PlayerActionError,
};
use crate::selection::SelectionState;
use crate::{CamelColor, CamelField, CamelState, GeneralWindow};
use crate::{
    camelfield::{ARROW_RIGHT, CAMEL_PATTERN},
    gameasset::GameAsset,
};

use calc::{CamelMap, Configuration, EffectCardType};
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
    pub selected: SelectionState,
    camel_round_info: [CamelState; 5],
    effect_card_info: [EffectCardState; 2],
    rolled_dice: usize,
    round_number: u8,
    pub game_period: GamePeriod,
    asset_manager: GameAssetManager,
}

impl Default for GameState {
    fn default() -> Self {
        let assets = asset_vec![
            ("arrow_right", ARROW_RIGHT),
            ("arrow_left", ARROW_LEFT),
            ("camel_pattern", CAMEL_PATTERN)
        ];
        let asset_manager = GameAssetManager::init_with_assets(assets);

        let mut fields: [CamelField; 16] = Default::default();

        for (i, field) in fields.iter_mut().enumerate() {
            field.index = (i + 2) % 16;
            field.board_index = i;
        }

        // first field in the game
        fields[0].selected = true;

        //default camel selected
        let mut camel_round_info = CamelColor::all().map(CamelState::new);
        camel_round_info[0].selected = true;

        let effect_card_info = [
            EffectCardState::new(EffectCardType::Oasis),
            EffectCardState::new(EffectCardType::Desert),
        ];

        Self {
            fields,
            selected: SelectionState::default(),
            camel_round_info,
            effect_card_info,
            rolled_dice: 0,
            game_period: GamePeriod::Setup,
            round_number: 0,
            asset_manager,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum GamePeriod {
    #[default]
    Setup,
    Game,
}

impl GameState {
    pub fn round_finished(&self) -> bool {
        self.rolled_dice == 5
    }

    pub fn add_camel(&mut self, selected_color: usize, selected_field: usize) {
        self.fields[selected_field].add_camel(selected_color.into());
    }

    pub fn convert_game_state_configuration(game_state: &GameState) -> Configuration {
        let positions =
            game_state
                .fields
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (idx, field)| {
                    if let Some(camels) = field.camels() {
                        let c: Vec<(u8, calc::Color)> = camels
                            .iter()
                            .map(|color| (idx as u8, Into::<calc::Color>::into(*color)))
                            .collect();
                        acc.extend(c);
                    }
                    acc
                });

        // Collect effect card placements
        let effect_cards: Vec<(usize, EffectCardType)> = game_state
            .effect_card_info
            .iter()
            .flat_map(|effect_state| {
                effect_state
                    .placements
                    .iter()
                    .map(|&pos| (pos as usize, effect_state.effect_type))
            })
            .collect();

        let colors: Vec<calc::Color> = game_state
            .camel_round_info
            .iter()
            .filter_map(|cam| {
                if !cam.has_moved {
                    Some(cam.camel_color.into())
                } else {
                    None
                }
            })
            .collect();

        // Build CamelMap with both positions and effect cards
        let camel_map = CamelMap::builder()
            .with_positions(positions)
            .with_effect_cards(effect_cards)
            .build();

        Configuration::builder()
            .with_camel_map(camel_map)
            .with_available_colors(colors)
            .build()
    }

    pub fn effect_placements(&self, effect: EffectCardType) -> &Vec<u8> {
        &self.effect_card_info[effect as usize].placements
    }

    pub fn add_dice_rolled(&mut self) {
        self.rolled_dice += 1;
    }

    pub fn init(config: &Vec<(u8, CamelColor)>) -> GameState {
        let mut init = GameState::default();

        init.fields[init.selected.field()].selected = true;
        init.camel_round_info[init.selected.color()].selected = true;

        for (i, col) in config {
            if let Some(camels) = init.fields[*i as usize].camels_mut() {
                camels.push(*col)
            }
            init.camel_round_info[*col as usize].start_pos = *i as u32;
        }

        init
    }

    pub fn move_camel(
        &mut self,
        camel: CamelColor,
        to_field: usize,
    ) -> Result<(), PlayerActionError> {
        let camel_state = self.camel_round_info[camel as usize];
        if to_field >= self.fields.len() {
            return Err(InvalidMove.into());
        }

        if camel_state.has_moved {
            return Err(InvalidMove.into());
        }

        let (old_pos, camel_index) = self.find_camel(camel).ok_or(InvalidConfiguration)?;

        let move_dist = if self
            .effect_placements(EffectCardType::Oasis)
            .contains(&(to_field.saturating_sub(1) as u8))
        {
            4
        } else {
            3
        };

        if self
            .effect_card_info
            .iter()
            .any(|card_type| card_type.placements.contains(&(to_field as u8)))
        {
            return Err(InvalidMove.into());
        }

        if old_pos > to_field && to_field - old_pos > move_dist {
            return Err(InvalidMove.into());
        }

        let move_camels_under = self
            .effect_placements(EffectCardType::Desert)
            .contains(&((to_field + 1) as u8));

        // only be able to move to same field if next field has desert effect card
        if old_pos == to_field && !move_camels_under {
            return Err(InvalidMove.into());
        }

        let camel_state = &mut self.camel_round_info[camel as usize];
        camel_state.has_moved = true;

        if let Some(old_camels) = self.fields[old_pos].camels_mut() {
            let mut moving_camels = old_camels.split_off(camel_index);

            for cam in &moving_camels {
                self.camel_round_info[*cam as usize].end_pos = to_field as u32;
            }

            match &mut self.fields[to_field].content {
                Some(CamelFieldContent::Camels(new_camels)) => {
                    // put moving camels under current ones
                    if move_camels_under {
                        moving_camels.append(new_camels);
                        std::mem::swap(new_camels, &mut moving_camels);
                    } else {
                        new_camels.extend(moving_camels);
                    }
                }
                Some(CamelFieldContent::EffectCard(_)) => {
                    // the player accounts for correct movement if the camel would've landed on a
                    // effect card
                }
                None => {
                    self.fields[to_field].content = Some(CamelFieldContent::Camels(moving_camels));
                }
            }
        }

        Ok(())
    }

    pub fn move_selected_field_rel(&mut self, by: i32) {
        let camel: CamelColor = self.selected.color().into();
        let old_idx = self.selected.field();
        let new_selection_idx = match old_idx as i32 + by {
            _idx @ ..0 => 15,
            _idx @ 16.. => 0,
            idx => idx as usize,
        };

        *self.selected.field_mut() = new_selection_idx;
        self.fields[old_idx].selected = false;
        self.fields[new_selection_idx].selected = true;

        // if current selection is effect card, camel info doesn't have to be updated
        if self.selected.item_type() == SelectionType::EffectCard {
            return;
        }

        if let Some((old_pos, camel_index)) = self.find_camel(camel)
            && let Some(old_camels) = self.fields[old_pos].camels()
        {
            let moving_camels = old_camels.iter().skip(camel_index);

            for cam in moving_camels {
                self.camel_round_info[*cam as usize].end_pos = new_selection_idx as u32;
            }
        };
    }

    pub fn move_selected_color(&mut self, new_color: usize) {
        // Clear any effect card selection first
        if self.selected.item_type() == SelectionType::EffectCard {
            self.effect_card_info[self.selected.effect() as usize].selected = false;
            *self.selected.item_type_mut() = SelectionType::Camel;
        }

        let old_color = self.selected.color();
        self.camel_round_info[old_color].selected = false;

        for cam_info in &mut self.camel_round_info {
            // TODO: maybe make this better
            if !cam_info.has_moved {
                cam_info.end_pos = cam_info.start_pos;
            }
        }

        self.camel_round_info[new_color].selected = true;
        *self.selected.color_mut() = new_color;
    }

    pub fn move_selected_effect(&mut self, new_effect: EffectCardType) {
        // Clear any camel selection first
        if self.selected.item_type() == SelectionType::Camel {
            self.camel_round_info[self.selected.color()].selected = false;
            *self.selected.item_type_mut() = SelectionType::EffectCard;
        }

        let old_effect = self.selected.effect();
        self.effect_card_info[old_effect as usize].selected = false;

        self.effect_card_info[new_effect as usize].selected = true;
        *self.selected.effect_mut() = new_effect;
    }

    fn find_camel(&self, camel: CamelColor) -> Option<(usize, usize)> {
        self.fields
            .iter()
            .enumerate()
            .find_map(|(field_idx, field)| {
                if let Some(camels) = field.camels() {
                    camels
                        .iter()
                        .position(|&c| c == camel)
                        .map(|camel_idx| (field_idx, camel_idx))
                } else {
                    None
                }
            })
    }

    pub fn move_selected_placement_type(&mut self, by: i32) {
        match *self.selected.item_type_mut() {
            SelectionType::Camel => {
                let old_idx = self.selected.color();
                let new_idx = old_idx as i32 + by;

                if new_idx < 0 {
                    // Wrap to Desert (bottom of effect cards)
                    self.camel_round_info[old_idx].selected = false;
                    *self.selected.item_type_mut() = SelectionType::EffectCard;
                    *self.selected.effect_mut() = EffectCardType::Desert;
                    self.effect_card_info[1].selected = true;
                } else if new_idx >= 5 {
                    // Move to Oasis (top of effect cards)
                    self.camel_round_info[old_idx].selected = false;
                    *self.selected.item_type_mut() = SelectionType::EffectCard;
                    *self.selected.effect_mut() = EffectCardType::Oasis;
                    self.effect_card_info[0].selected = true;
                } else {
                    // Stay in camels
                    self.camel_round_info[old_idx].selected = false;
                    *self.selected.color_mut() = new_idx as usize;
                    self.camel_round_info[new_idx as usize].selected = true;
                }
            }
            SelectionType::EffectCard => {
                let old_effect = self.selected.effect();
                let mut new_idx = old_effect as i32 + by;

                if new_idx < 0 {
                    // Move to bottom camel
                    self.effect_card_info[old_effect as usize].selected = false;
                    *self.selected.item_type_mut() = SelectionType::Camel;
                    *self.selected.color_mut() = 4;
                    self.camel_round_info[4].selected = true;
                } else if new_idx >= 2 {
                    // Wrap to top camel
                    self.effect_card_info[old_effect as usize].selected = false;
                    *self.selected.item_type_mut() = SelectionType::Camel;
                    *self.selected.color_mut() = 0;
                    self.camel_round_info[0].selected = true;
                } else {
                    self.effect_card_info[old_effect as usize].selected = false;
                    new_idx = new_idx.rem_euclid(self.effect_card_info.len() as i32);
                    *self.selected.effect_mut() = EffectCardType::from_usize(new_idx as usize);
                    self.effect_card_info[new_idx as usize].selected = true;
                }
            }
        }
    }

    pub fn new_round(&mut self) {
        self.round_number += 1;
        for cam_info in &mut self.camel_round_info {
            cam_info.start_pos = cam_info.end_pos;
            cam_info.has_moved = false;
        }
        // Clear effect card placements
        for effect_info in &mut self.effect_card_info {
            effect_info.clear_placements();
        }
        // Clear effects from fields
        for field in &mut self.fields {
            field.clear_effects();
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
            .title_top("Camels");

        let inner_area = outer_line.inner(area);

        outer_line.render(area, buf);

        let constraints = [
            vec![Constraint::Length(1)],    // "Round X"
            vec![Constraint::Length(4); 5], // 5 camels
            vec![Constraint::Length(3); 2], // 2 effect card types
            vec![Constraint::Length(1)],    // "Remaining Dice"
        ]
        .concat();

        let camel_layout = Layout::vertical(constraints);

        let rows: [_; 9] = camel_layout.areas(inner_area);

        let round_string = if self.round_number == 0 {
            "Initialization Round".to_string()
        } else {
            format!("Round {}", self.round_number)
        };

        Line::from(round_string).centered().render(rows[0], buf);

        // Render camels (rows 1-5)
        for (i, r) in rows.iter().take(6).skip(1).enumerate() {
            self.camel_round_info[i].render(*r, buf);
        }

        // Render effect cards (rows 6-7)
        for (i, r) in rows.iter().take(8).skip(6).enumerate() {
            (&self.effect_card_info[i]).render(*r, buf);
        }

        Line::from(format!("Remaining Dice {dice}/5", dice = self.rolled_dice))
            .centered()
            .render(rows[8], buf);
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
            field.render(camel_field_areas[(i + 2) % 16], buf, &self.asset_manager);
        }
    }

    pub(crate) fn camel_info(&mut self, camel_color: usize) -> Option<&mut CamelState> {
        self.camel_round_info.get_mut(camel_color)
    }

    pub fn toggle_effect_card(
        &mut self,
        effect_idx: EffectCardType,
        field: usize,
    ) -> Result<(), ()> {
        if let Some(CamelFieldContent::Camels(_)) = self.fields[field].content {
            return Err(());
        }

        let was_placed = self.effect_card_info[effect_idx as usize].has_placement(field as u8);
        self.effect_card_info[effect_idx as usize].toggle_placement(field as u8);

        let effect_type = self.effect_card_info[effect_idx as usize].effect_type;
        if was_placed {
            self.fields[field].remove_effect(effect_type);
        } else {
            self.fields[field].add_effect(effect_type);
        }
        Ok(())
    }

    pub fn effect_card_info(&mut self, idx: usize) -> Option<&mut EffectCardState> {
        self.effect_card_info.get_mut(idx)
    }
}
