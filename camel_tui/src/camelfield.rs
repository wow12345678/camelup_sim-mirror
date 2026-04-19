use std::{cmp, fmt::Display};

use calc::EffectCardType;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

use crate::gameasset::GameAssetManager;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
// CamelColor with first as default
pub enum CamelColor {
    #[default]
    Blue = 0,
    Green = 1,
    Orange = 2,
    White = 3,
    Yellow = 4,
}

impl From<calc::Color> for CamelColor {
    fn from(value: calc::Color) -> Self {
        match value {
            calc::Color::Blue => CamelColor::Blue,
            calc::Color::Green => CamelColor::Green,
            calc::Color::Orange => CamelColor::Orange,
            calc::Color::White => CamelColor::White,
            calc::Color::Yellow => CamelColor::Yellow,
        }
    }
}

impl From<CamelColor> for calc::Color {
    fn from(value: CamelColor) -> Self {
        match value {
            CamelColor::Blue => calc::Color::Blue,
            CamelColor::Green => calc::Color::Green,
            CamelColor::Orange => calc::Color::Orange,
            CamelColor::White => calc::Color::White,
            CamelColor::Yellow => calc::Color::Yellow,
        }
    }
}

impl Display for CamelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CamelColor::Blue => write!(f, "Blue"),
            CamelColor::Green => write!(f, "Green"),
            CamelColor::Yellow => write!(f, "Yellow"),
            CamelColor::Orange => write!(f, "Orange"),
            CamelColor::White => write!(f, "White"),
        }
    }
}

impl From<usize> for CamelColor {
    fn from(value: usize) -> Self {
        match value {
            0 => CamelColor::Blue,
            1 => CamelColor::Green,
            2 => CamelColor::Orange,
            3 => CamelColor::White,
            4 => CamelColor::Yellow,
            _ => panic!("Not a valid value for Color conversion"),
        }
    }
}

impl CamelColor {
    pub fn all() -> [CamelColor; 5] {
        [
            CamelColor::Blue,
            CamelColor::Green,
            CamelColor::Orange,
            CamelColor::White,
            CamelColor::Yellow,
        ]
    }

    pub fn from_char_to_usize(c: char) -> usize {
        match c {
            'b' => 0,
            'g' => 1,
            'o' => 2,
            'w' => 3,
            'y' => 4,
            _ => panic!("invalid character"),
        }
    }

    // was meant for text color when background is different, so that it is still readable
    pub const fn text_color(self) -> Color {
        match self {
            CamelColor::Blue => Color::Rgb(255, 255, 255),
            CamelColor::Green => Color::Rgb(255, 255, 255),
            CamelColor::Yellow => Color::Rgb(255, 255, 255),
            CamelColor::Orange => Color::Rgb(255, 255, 255),
            CamelColor::White => Color::Rgb(255, 255, 255),
        }
    }

    pub const fn to_color(self) -> Color {
        match self {
            CamelColor::Blue => Color::Rgb(52, 18, 237),
            CamelColor::Green => Color::Rgb(0, 255, 0),
            CamelColor::Yellow => Color::Rgb(255, 255, 0),
            CamelColor::Orange => Color::Rgb(255, 165, 0),
            CamelColor::White => Color::Rgb(255, 255, 255),
        }
    }
}

#[rustfmt::skip]
pub const CAMEL_PATTERN: [[bool; 19]; 15] = [
    [false, false, false, false, false, false, false,false, false, false, false, false, false, false, false, true, true,  false, false ],
    [false, false, false, false, false, false, false,false, false, false, false, false, false, false, true,  true, true,  true,  true  ],
    [false, false, false, false, false, false, true, true,  true,  true,  false, false, false, false, true,  true, true,  true,  true  ],
    [false, false, false, false, false, true,  true, true,  true,  true,  true,  false, false, false, true,  true, true,  false, false ],
    [false, false, false, false, true,  true,  true, true,  true,  true,  true,  true,  false, false, true,  true, true,  false, false ],
    [false, false, false, true,  true,  true,  true, true,  true,  true,  true,  true,  true,  false, true,  true, true,  false, false ],
    [false, true,  true,  true,  true,  true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true, true,  false, false ],
    [true , false, true,  true,  true,  true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true, true,  false, false ],
    [true , false, true,  true,  true,  true,  true, true,  true,  true,  true,  true,  true,  true,  true,  true, true,  false, false ],
    [true , false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [true , false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  false, false, true, false, false, false, false, false, true,  false, false, true, false, false, false ],
    [false, false, false, true,  true,  false, true, true,  false, false, false, false, true,  true,  false, true, true,  false, false ],
];

#[rustfmt::skip]
pub const ARROW_RIGHT: [[bool; 20]; 10] = [
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,  false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, true,  false, false, false, true,  true,  false, false, false],
    [true,  true,  true,  true,  true,  false, false, false, false, false, true,  true,  false, false, true,  true,  true,  true,  false, false],
    [true,  false, false, false, false, false, false, true,  false, false, false, true,  false, false, false, false, false, true,  true,  false],
    [true,  false, false, false, false, false, true,  true,  true,  false, false, true,  false, false, false, false, false, false, true,  true,],
    [true,  false, false, false, false, false, false, true,  false, false, false, true,  false, false, false, false, false, true,  true,  false],
    [true,  true,  true,  true,  true,  false, false, false, false, false, true,  true,  true,  false, true,  true,  true,  true,  false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,  true,  false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,  false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
];

#[rustfmt::skip]
pub const ARROW_LEFT: [[bool; 20]; 10] = [
    [false, false, false, false, true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, true,  true,  false, false, false, false, false, false, false, true,  false, false, false, false, false, false, false],
    [false, false, true,  true,  true,  true,  false, false, false, false, false, true,  true,  false, false, true,  true,  true,  true,  true,],
    [false, true,  true,  false, false, false, false, false, false, false, false, false, true,  false, false, false, false, false, false, true,],
    [true,  true,  false, false, false, false, false, true,  true,  true,  false, false, true,  false, false, false, false, false, false, true,],
    [false, true,  true,  false, false, false, false, false, false, false, false, false, true,  false, false, false, false, false, false, true,],
    [false, false, true,  true,  true,  true,  false, false, false, false, false, true,  true,  true,  false, true,  true,  true,  true,  true,],
    [false, false, false, true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
];

/// camels are ordered from bottom to top
/// index is the index in the representation in GameField
/// board_index is the index that is rendered for the field
/// board representation:
///  ______________
/// |14|15| 0| 1| 2|
/// |13|‾‾‾‾‾‾‾‾| 3|
/// |12|        | 4|
/// |11|________| 5|
/// |10| 9| 8| 7| 6|
///  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾
#[derive(Debug, Default)]
pub struct CamelField {
    pub content: Option<CamelFieldContent>,
    pub board_index: usize,
    pub index: usize,
    pub selected: bool,
}

impl CamelField {
    #[allow(unused)]
    fn new(content: CamelFieldContent, index: usize, board_index: usize) -> Self {
        Self {
            content: Some(content),
            board_index,
            index,
            selected: false,
        }
    }

    pub fn camels(&self) -> Option<&Vec<CamelColor>> {
        match &self.content {
            Some(CamelFieldContent::Camels(camels)) => Some(camels),
            Some(CamelFieldContent::EffectCard(_)) => None,
            None => None,
        }
    }

    pub fn camels_mut(&mut self) -> Option<&mut Vec<CamelColor>> {
        match &mut self.content {
            Some(CamelFieldContent::Camels(camels)) => Some(camels),
            Some(CamelFieldContent::EffectCard(_)) => None,
            None => None,
        }
    }

    pub fn add_camel(&mut self, new_camel: CamelColor) {
        match &mut self.content {
            Some(CamelFieldContent::Camels(camels)) => {
                camels.push(new_camel);
            }
            Some(CamelFieldContent::EffectCard(_)) => {
                // do nothing because camels cannot be on fields with effects on them
            }
            None => {
                self.content
                    .replace(CamelFieldContent::Camels(vec![new_camel]));
            }
        }
    }

    pub fn add_effect(&mut self, new_effect: EffectCardType) {
        match &mut self.content {
            Some(CamelFieldContent::Camels(_)) => {
                // do nothing because effects cannot be placed on fields with camels on them
            }
            Some(CamelFieldContent::EffectCard(_)) | None => {
                self.content
                    .replace(CamelFieldContent::EffectCard(new_effect));
            }
        }
    }

    pub fn remove_effect(&mut self, effect_to_remove: EffectCardType) {
        if let Some(CamelFieldContent::EffectCard(current)) = &self.content
            && *current == effect_to_remove
        {
            self.content = None;
        }
    }

    pub fn clear_effects(&mut self) {
        if matches!(&self.content, Some(CamelFieldContent::EffectCard(_))) {
            self.content = None;
        }
    }

    fn render_camels(
        &self,
        area: Rect,
        buf: &mut Buffer,
        camels: &[CamelColor],
        asset_manager: &GameAssetManager,
    ) {
        let asset = asset_manager
            .get_asset("camel_pattern")
            .expect("asset should be initialized");

        // Account for borders (1 char each side)
        let inner_x = area.x + 1;
        let inner_width = area.width.saturating_sub(2);
        let inner_height = area.height.saturating_sub(2);
        let field_bottom = area.y + area.height - 1;

        // Flip camel direction based on which half of the board they're on
        let flip = (8..16).contains(&self.index);

        // Scale camel to fit within field height with some padding (1.3x factor)
        let camel_pixel_height = asset.view().height();
        let camel_char_height = camel_pixel_height.div_ceil(2) as u16;
        let scale = (area.height as f32) / (1.3 * camel_char_height as f32);

        // Calculate vertical stacking offset to distribute camels evenly
        let total_camels = camels.len();
        let camel_stacking_offset = if total_camels == 0 {
            0
        } else {
            cmp::max(1, inner_height / (total_camels as u16 + 1))
        };

        for (camel_idx, camel) in camels.iter().enumerate() {
            let view = asset.view().scale(scale).flip(flip);

            let camel_width = view.width() as u16;
            let camel_height = view.height().div_ceil(2) as u16;

            // Center horizontally within the inner area
            let x = inner_x + (inner_width.saturating_sub(camel_width)) / 2;

            // Stack camels from bottom, with each camel offset upward
            // TODO: consider using total_camels - camel_idx - 1 for reverse stacking
            let y = field_bottom - camel_height - (camel_idx as u16 * camel_stacking_offset);

            let drawing_area = Rect::new(x, y, camel_width, camel_height);

            view.render(drawing_area, buf, camel.to_color());
        }
    }
}

impl CamelField {
    pub fn render(&self, area: Rect, buf: &mut Buffer, asset_manager: &GameAssetManager) {
        let border_color = if self.selected {
            Color::LightBlue
        } else {
            Color::White
        };

        if cfg!(debug_assertions) {
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_top(format!(
                    " Field {}, area:{},{},{},{} ",
                    self.board_index, area.x, area.y, area.width, area.height
                ))
                .render(area, buf);
        } else {
            Block::bordered()
                .border_style(Style::default().fg(border_color))
                .title_top(format!(" Field {} ", self.board_index))
                .render(area, buf);
        }

        match &self.content {
            Some(CamelFieldContent::Camels(camels)) => {
                self.render_camels(area, buf, camels, asset_manager);
            }
            Some(CamelFieldContent::EffectCard(effect)) => {
                let asset = if let EffectCardType::Oasis = effect {
                    asset_manager
                        .get_asset("arrow_right")
                        .expect("asset should be initialized")
                } else {
                    asset_manager
                        .get_asset("arrow_left")
                        .expect("asset should be initialized")
                };

                let scale = (area.height as f32) / (asset.view().height() as f32);
                let view = asset.view().scale(scale);

                let arrow_width = view.width() as u16;
                let arrow_height = view.height().div_ceil(2) as u16;

                let inner_x = area.x + 1;
                let inner_y = area.y + 1;
                let inner_width = area.width.saturating_sub(2);
                let inner_height = area.height.saturating_sub(2);

                let drawing_area = Rect::new(
                    inner_x + (inner_width.saturating_sub(arrow_width)) / 2,
                    inner_y + (inner_height.saturating_sub(arrow_height)) / 2,
                    arrow_width,
                    arrow_height,
                );

                view.render(drawing_area, buf, Color::from(effect.to_color()));
            }
            None => {}
        }
    }
}

#[derive(Debug)]
pub enum CamelFieldContent {
    Camels(Vec<CamelColor>),
    EffectCard(EffectCardType),
}
